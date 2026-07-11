//! `crawlingo` — the native Rust command-line interface for the Crawlingo engine.
//!
//! ## Subcommands
//!
//! | Command  | Description |
//! |----------|-------------|
//! | `fetch`  | Fetch a single URL and print the page HTML or markdown. |
//! | `crawl`  | Multi-page crawl: follow links, extract fields, output JSON/CSV. |
//! | `sitemap`| Parse and list all URLs in a sitemap (or sitemap index). |
//! | `download` | Download a file to disk, with optional resume. |
//!
//! ## Configuration
//! All subcommands read `CRAWLINGO_*` environment variables for defaults (proxy, rate limit, etc.)
//! via [`crawlingo::config::CrawlingoConfig::load`]. Command-line flags always override them.

#[cfg(feature = "cli")]
fn main() {
    use clap::{Parser, Subcommand};
    use crawlingo::config::CrawlingoConfig;
    use crawlingo::crawl::crawler::Crawler;
    use crawlingo::crawl::sitemap::{parse_sitemap, ParsedSitemap};
    use crawlingo::dataset::builder::DatasetField;
    use crawlingo::engine::download::Downloader;
    use crawlingo::engine::session::Session;
    use std::sync::Arc;

    #[derive(Parser)]
    #[command(
        name = "crawlingo",
        version = env!("CARGO_PKG_VERSION"),
        about = "Crawlingo — stealth web scraping and crawling engine",
        long_about = None,
    )]
    struct Cli {
        /// Path to a TOML or JSON config file. Merged with CRAWLINGO_* env vars.
        #[arg(long, global = true, env = "CRAWLINGO_CONFIG")]
        config: Option<std::path::PathBuf>,

        /// Proxy URL (overrides config). E.g. `http://user:pass@host:port`.
        #[arg(long, global = true, env = "CRAWLINGO_PROXY")]
        proxy: Option<String>,

        /// Requests per second rate limit (overrides config).
        #[arg(long, global = true, env = "CRAWLINGO_RATE_LIMIT_RPS")]
        rate_limit_rps: Option<f64>,

        /// Request timeout in seconds (overrides config).
        #[arg(
            long,
            global = true,
            env = "CRAWLINGO_TIMEOUT_SECONDS",
            default_value = "30"
        )]
        timeout: u64,

        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Fetch a single URL and print its content.
        Fetch {
            /// The URL to fetch.
            url: String,

            /// Print as Markdown instead of raw HTML.
            #[arg(long)]
            markdown: bool,

            /// Use the stealthy browser-fingerprint fetcher tier.
            #[arg(long)]
            stealthy: bool,
        },

        /// Crawl multiple pages, extract fields, and output results as JSON.
        Crawl {
            /// The starting URL.
            url: String,

            /// CSS selector of links to follow (e.g. `a`).
            #[arg(long, default_value = "a")]
            follow: String,

            /// Maximum number of pages to crawl.
            #[arg(long, default_value = "10")]
            limit: usize,

            /// Maximum crawl depth.
            #[arg(long, default_value = "3")]
            depth: usize,

            /// Number of concurrent workers.
            #[arg(long, default_value = "2")]
            concurrency: usize,

            /// Politeness delay between requests, in seconds.
            #[arg(long, default_value = "0")]
            delay: f64,

            /// Field definitions as `name:css_selector` pairs (repeatable).
            /// E.g. `--field title:h1 --field price:.price`
            #[arg(long = "field", value_name = "NAME:SELECTOR")]
            fields: Vec<String>,

            /// Output format: `json` (default) or `csv`.
            #[arg(long, default_value = "json")]
            output: String,

            /// Persist crawl state to this path for resumable crawling.
            #[arg(long)]
            resume: Option<std::path::PathBuf>,
        },

        /// Fetch and parse a sitemap, listing all discovered URLs.
        Sitemap {
            /// The sitemap URL (or page URL — crawlingo will append `/sitemap.xml`).
            url: String,

            /// Print only the loc URLs (one per line) instead of formatted output.
            #[arg(long)]
            urls_only: bool,

            /// Maximum sitemap-index nesting depth.
            #[arg(long, default_value = "5")]
            max_depth: usize,
        },

        /// Download a file from a URL to disk.
        Download {
            /// The URL to download.
            url: String,

            /// Destination file path.
            #[arg(long, short)]
            output: std::path::PathBuf,

            /// Skip resuming a partial download (always restart from byte 0).
            #[arg(long)]
            no_resume: bool,

            /// Maximum bytes to download.
            #[arg(long)]
            max_bytes: Option<u64>,
        },
    }

    let cli = Cli::parse();

    // Initialize tracing to stderr.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .compact()
        .init();

    // Load config (file + env), then build Session.
    let config_path = cli.config.as_deref();
    let mut config = CrawlingoConfig::load(config_path).unwrap_or_else(|e| {
        eprintln!("warning: failed to load config: {e}; using defaults");
        CrawlingoConfig::default()
    });

    // Apply CLI overrides on top of config.
    if let Some(proxy) = cli.proxy {
        config.proxy = Some(proxy);
    }
    if let Some(rps) = cli.rate_limit_rps {
        config.rate_limit_rps = rps;
    }
    config.timeout_seconds = cli.timeout;

    let session = Arc::new(Session::from_config(&config));

    match cli.command {
        Commands::Fetch {
            url,
            markdown,
            stealthy,
        } => {
            use crawlingo::engine::fetcher::{FetchRequest, FetcherTier};
            use crawlingo::parser::streaming::HtmlParser;

            let req = FetchRequest {
                url: url.clone(),
                tier: if stealthy {
                    FetcherTier::Stealthy
                } else {
                    FetcherTier::Standard
                },
                browser_profile: None,
                headers: session.headers.read().unwrap().clone(),
                cookies: session.cookies.read().unwrap().clone(),
                proxy: session.get_next_proxy(),
                timeout: std::time::Duration::from_secs(cli.timeout),
                retries: 2,
                rate_limit_rps: config.rate_limit_rps,
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let result = rt.block_on(async {
                let manager = session.fetch_manager();
                let resp = manager.dispatch(req).await?;
                let page = HtmlParser::parse(resp)?;
                Ok::<_, crawlingo::error::CrawlingoError>(page)
            });

            match result {
                Ok(page) => {
                    if markdown {
                        println!(
                            "{}",
                            crawlingo::parser::document::Page::render_markdown(page.dom_tree())
                        );
                    } else {
                        println!("{}", page.html());
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Crawl {
            url,
            follow,
            limit,
            depth,
            concurrency,
            delay,
            fields,
            output,
            resume,
        } => {
            // Parse field definitions: "name:selector"
            let dataset_fields: Vec<DatasetField> = fields
                .iter()
                .filter_map(|f| {
                    let mut parts = f.splitn(2, ':');
                    let name = parts.next()?.trim().to_string();
                    let selector = parts.next()?.trim().to_string();
                    Some(DatasetField {
                        name,
                        selector,
                        selector_type: "css".to_string(),
                        #[cfg(feature = "python")]
                        transform: None,
                        default: None,
                        extract_type: Default::default(),
                    })
                })
                .collect();

            let crawler_result = if let Some(resume_path) = resume {
                Crawler::resumable(&url, session.clone(), &resume_path)
            } else {
                Ok(Crawler::new(&url, session.clone()))
            };

            let mut crawler = match crawler_result {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            };
            crawler.follow_selector = follow;
            crawler.limit = limit;
            crawler.max_depth = depth;
            crawler.concurrency = concurrency;
            crawler.delay_seconds = delay;
            crawler.fields = dataset_fields;

            match crawler.crawl() {
                Ok(results) => {
                    if output == "csv" {
                        // Simple CSV output.
                        if results.is_empty() {
                            eprintln!("No results.");
                            return;
                        }
                        // Header
                        let headers: Vec<String> = {
                            let mut h = vec!["url".to_string(), "timestamp".to_string()];
                            h.extend(results[0].fields.keys().cloned());
                            h
                        };
                        println!("{}", headers.join(","));
                        for r in &results {
                            let mut row =
                                vec![csv_escape(&r.url), csv_escape(&r.timestamp.to_rfc3339())];
                            for key in headers.iter().skip(2) {
                                row.push(csv_escape(
                                    r.fields.get(key).map(String::as_str).unwrap_or(""),
                                ));
                            }
                            println!("{}", row.join(","));
                        }
                    } else {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&results).unwrap_or_default()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Sitemap {
            url,
            urls_only,
            max_depth,
        } => {
            // If the URL looks like a page rather than a sitemap, try appending /sitemap.xml.
            let sitemap_url = if url.ends_with(".xml") || url.ends_with(".xml.gz") {
                url.clone()
            } else {
                crawlingo::crawl::sitemap::sitemap_url_for_origin(&url)
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let result = rt.block_on(async {
                let manager = session.fetch_manager();
                let req = crawlingo::engine::fetcher::FetchRequest {
                    url: sitemap_url.clone(),
                    tier: crawlingo::engine::fetcher::FetcherTier::Standard,
                    browser_profile: None,
                    headers: session.headers.read().unwrap().clone(),
                    cookies: session.cookies.read().unwrap().clone(),
                    proxy: session.get_next_proxy(),
                    timeout: std::time::Duration::from_secs(cli.timeout),
                    retries: 2,
                    rate_limit_rps: 0.0,
                };
                let resp = manager.dispatch(req).await?;
                Ok::<_, crawlingo::error::CrawlingoError>(resp.body.to_vec())
            });

            match result {
                Ok(ref xml_vec) => {
                    let _ = max_depth; // Used when doing recursive index, not in this simplified path
                    match parse_sitemap(xml_vec.as_slice()) {
                        Ok(ParsedSitemap::Urlset(entries)) => {
                            if urls_only {
                                for e in &entries {
                                    println!("{}", e.loc);
                                }
                            } else {
                                eprintln!("Sitemap: {} ({} URLs)", sitemap_url, entries.len());
                                for e in &entries {
                                    let meta = [
                                        e.lastmod.as_deref().map(|v| format!("lastmod={v}")),
                                        e.changefreq.as_deref().map(|v| format!("changefreq={v}")),
                                        e.priority.as_deref().map(|v| format!("priority={v}")),
                                    ]
                                    .into_iter()
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                    if meta.is_empty() {
                                        println!("{}", e.loc);
                                    } else {
                                        println!("{} [{}]", e.loc, meta);
                                    }
                                }
                            }
                        }
                        Ok(ParsedSitemap::Index(entries)) => {
                            if urls_only {
                                for e in &entries {
                                    println!("{}", e.loc);
                                }
                            } else {
                                eprintln!(
                                    "Sitemap index: {} ({} child sitemaps)",
                                    sitemap_url,
                                    entries.len()
                                );
                                for e in &entries {
                                    println!("{}", e.loc);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("error parsing sitemap: {e}");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error fetching sitemap: {e}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Download {
            url,
            output,
            no_resume,
            max_bytes,
        } => {
            let mut dl = Downloader::new(session).with_resume(!no_resume);
            if let Some(max) = max_bytes {
                dl = dl.with_max_bytes(max);
            }
            match dl.download_to_file(&url, &output) {
                Ok(result) => {
                    eprintln!(
                        "Downloaded {} bytes ({}) → {} [HTTP {}]{}",
                        result.bytes_written,
                        result.content_type,
                        output.display(),
                        result.status,
                        if result.resumed { " (resumed)" } else { "" },
                    );
                    if let Some(fname) = result.suggested_filename {
                        eprintln!("Filename hint: {fname}");
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Minimal CSV cell escaping: wrap in quotes and double any internal quotes.
#[cfg(feature = "cli")]
fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("This binary requires the 'cli' feature. Build with: cargo build --features cli");
    std::process::exit(1);
}
