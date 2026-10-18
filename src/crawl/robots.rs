//! Robots.txt parsing, User-agent evaluation, and host-level rule caching.

use crate::engine::fetcher::{FetchRequest, FetcherTier};
use crate::engine::session::Session;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// Directive rule (Allow or Disallow).
#[derive(Debug, Clone)]
pub struct Rule {
    pub path: String,
    pub allow: bool,
}

/// Rule block associated with one or more User-agent headers.
#[derive(Debug, Clone, Default)]
pub struct UserAgentGroup {
    pub agents: Vec<String>,
    pub rules: Vec<Rule>,
    pub crawl_delay: Option<f64>,
}

/// Parsed representation of a robots.txt document.
#[derive(Debug, Clone, Default)]
pub struct RobotsRules {
    pub groups: Vec<UserAgentGroup>,
    pub sitemaps: Vec<String>,
}

impl RobotsRules {
    /// Creates a permissive ruleset that allows all paths.
    pub fn allow_all() -> Self {
        Self::default()
    }

    /// Parses a robots.txt body string.
    pub fn parse(content: &str) -> Self {
        let mut groups = Vec::new();
        let mut sitemaps = Vec::new();

        let mut current_agents: Vec<String> = Vec::new();
        let mut current_rules: Vec<Rule> = Vec::new();
        let mut current_delay: Option<f64> = None;

        for line in content.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or("").trim().to_lowercase();
            let val = parts.next().unwrap_or("").trim();

            match key.as_str() {
                "user-agent" => {
                    // If we were building rules under previous agents and hit a new user-agent, save previous group
                    if !current_rules.is_empty() || current_delay.is_some() {
                        groups.push(UserAgentGroup {
                            agents: current_agents.clone(),
                            rules: current_rules.clone(),
                            crawl_delay: current_delay,
                        });
                        current_agents.clear();
                        current_rules.clear();
                        current_delay = None;
                    }
                    current_agents.push(val.to_lowercase());
                }
                "disallow" => {
                    if !val.is_empty() {
                        current_rules.push(Rule {
                            path: val.to_string(),
                            allow: false,
                        });
                    }
                }
                "allow" => {
                    if !val.is_empty() {
                        current_rules.push(Rule {
                            path: val.to_string(),
                            allow: true,
                        });
                    }
                }
                "crawl-delay" => {
                    if let Ok(d) = val.parse::<f64>() {
                        if d.is_finite() && d >= 0.0 {
                            current_delay = Some(d);
                        }
                    }
                }
                "sitemap" if !val.is_empty() => {
                    sitemaps.push(val.to_string());
                }
                _ => {}
            }
        }

        if !current_agents.is_empty() || !current_rules.is_empty() {
            groups.push(UserAgentGroup {
                agents: current_agents,
                rules: current_rules,
                crawl_delay: current_delay,
            });
        }

        Self { groups, sitemaps }
    }

    /// Evaluates if `url_path` (e.g. `/admin/dashboard`) is allowed for `user_agent`.
    pub fn is_allowed(&self, user_agent: &str, url_path: &str) -> bool {
        let ua_lower = user_agent.to_lowercase();

        // 1. Find matching group for user_agent, or fallback to "*"
        let specific = self
            .groups
            .iter()
            .any(|g| g.agents.iter().any(|a| a != "*" && ua_lower.contains(a)));

        // Longest matching path rule takes precedence
        let path = if url_path.is_empty() { "/" } else { url_path };

        let mut longest_match_len = 0;
        let mut allowed = true;
        let mut matched = false;

        for group in self.groups.iter().filter(|g| {
            g.agents.iter().any(|a| {
                if specific {
                    a != "*" && ua_lower.contains(a)
                } else {
                    a == "*"
                }
            })
        }) {
            for rule in &group.rules {
                let length = rule
                    .path
                    .bytes()
                    .filter(|c| *c != b'*' && *c != b'$')
                    .count();
                if path_matches(&rule.path, path)
                    && (!matched
                        || length > longest_match_len
                        || length == longest_match_len && rule.allow)
                {
                    matched = true;
                    longest_match_len = length;
                    allowed = rule.allow;
                }
            }
        }

        allowed
    }

    /// Returns the crawl-delay setting (if any) for `user_agent`.
    pub fn crawl_delay(&self, user_agent: &str) -> Option<f64> {
        let ua_lower = user_agent.to_lowercase();
        let specific = self
            .groups
            .iter()
            .any(|g| g.agents.iter().any(|a| a != "*" && ua_lower.contains(a)));
        self.groups
            .iter()
            .filter(|g| {
                g.agents.iter().any(|a| {
                    if specific {
                        a != "*" && ua_lower.contains(a)
                    } else {
                        a == "*"
                    }
                })
            })
            .filter_map(|g| g.crawl_delay)
            .max_by(f64::total_cmp)
    }
}

/// Path pattern matcher with simple wildcard `*` support.
fn path_matches(pattern: &str, path: &str) -> bool {
    if pattern.is_empty() || pattern == "/" {
        return true;
    }
    let anchored = pattern.ends_with('$');
    let pattern = pattern.strip_suffix('$').unwrap_or(pattern);
    if !pattern.contains('*') {
        return if anchored {
            path == pattern
        } else {
            path.starts_with(pattern)
        };
    }

    // Remember the latest wildcard so an anchored suffix can match a later
    // occurrence, e.g. /*a$ must match /aba as well as /a.
    let (pattern, path) = (pattern.as_bytes(), path.as_bytes());
    let (mut p, mut s) = (0, 0);
    let mut star = None;
    let mut retry = 0;
    while s < path.len() {
        if p == pattern.len() && !anchored {
            return true;
        }
        if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = s;
        } else if p < pattern.len() && pattern[p] == path[s] {
            p += 1;
            s += 1;
        } else if let Some(index) = star {
            retry += 1;
            s = retry;
            p = index + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

/// Host-level cache for parsed `RobotsRules`.
pub struct RobotsCache {
    cache: moka::future::Cache<String, Arc<RobotsRules>>,
    pacing: crate::engine::rate_limiter::HostRateLimiter,
}

impl RobotsCache {
    pub fn new() -> Self {
        Self {
            cache: moka::future::Cache::builder()
                .max_capacity(4096)
                .time_to_live(Duration::from_secs(3600))
                .build(),
            pacing: Default::default(),
        }
    }

    /// Returns host authority string (e.g. `example.com:80`) for `url_str`.
    fn host_key(url_str: &str) -> Option<String> {
        let parsed = Url::parse(url_str).ok()?;
        Some(parsed.origin().ascii_serialization())
    }

    /// Fetches and caches `robots.txt` for `url_str` using `session.fetch_manager()`.
    pub async fn get_or_fetch(&self, session: &Session, url_str: &str) -> Arc<RobotsRules> {
        let key = match Self::host_key(url_str) {
            Some(k) => k,
            None => return Arc::new(RobotsRules::allow_all()),
        };

        self.cache
            .get_with(key.clone(), async {
                // Build robots.txt URL
                let parsed = match Url::parse(url_str) {
                    Ok(u) => u,
                    Err(_) => return Arc::new(RobotsRules::allow_all()),
                };

                let robots_url = parsed.join("/robots.txt").unwrap().to_string();

                let manager = session.fetch_manager();
                let mut headers = session.read_headers();
                if !headers
                    .keys()
                    .any(|key| key.eq_ignore_ascii_case("user-agent"))
                {
                    headers.insert("User-Agent".into(), "CrawlingoBot".into());
                }
                let req = FetchRequest {
                    url: robots_url,
                    tier: FetcherTier::Standard,
                    browser_profile: None,
                    headers,
                    cookies: HashMap::new(),
                    proxy: session.get_next_proxy_with_auth(),
                    timeout: Duration::from_secs(10),
                    retries: 1,
                };

                let rules = match manager.dispatch(req).await {
                    Ok(resp) if resp.status == 200 => {
                        let body = String::from_utf8_lossy(&resp.body);
                        Arc::new(RobotsRules::parse(&body))
                    }
                    Ok(resp) if resp.status == 404 || resp.status == 410 => {
                        Arc::new(RobotsRules::allow_all())
                    }
                    _ => Arc::new(RobotsRules::parse("User-agent: *\nDisallow: /")),
                };
                rules
            })
            .await
    }

    pub async fn wait_delay(&self, url: &str, rules: &RobotsRules, agent: &str) {
        if let Some(delay) = rules.crawl_delay(agent).filter(|delay| *delay > 0.0) {
            if let Some(key) = Self::host_key(url) {
                self.pacing.wait(&key, 1.0 / delay).await;
            }
        }
    }
}

impl Default for RobotsCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_only_disallow_and_anchored_suffix() {
        let rules = RobotsRules::parse("User-agent: *\nDisallow: *");
        assert!(!rules.is_allowed("CrawlingoBot", "/anything"));
        assert!(path_matches("/*a$", "/aba"));
        assert!(!path_matches("/*a$", "/ab"));
        assert!(path_matches("/*.pdf$", "/a.pdf.pdf"));
        assert!(!path_matches("/*.pdf$", "/a.pdf?download=1"));
    }

    #[test]
    fn merged_groups_comments_and_allow_ties() {
        let rules = RobotsRules::parse("User-agent: CrawlingoBot\nDisallow: /secret # private\nUser-agent: CrawlingoBot\nAllow: /secret\nDisallow: /other\nUser-agent: *\nDisallow: /");
        assert!(rules.is_allowed("CrawlingoBot", "/secret"));
        assert!(!rules.is_allowed("CrawlingoBot", "/other"));
        assert!(rules.is_allowed("CrawlingoBot", "/public"));
        assert!(!rules.is_allowed("OtherBot", "/public"));
    }

    #[test]
    fn parses_allow_and_disallow_rules() {
        let content = r#"
User-agent: *
Disallow: /admin/
Disallow: /private/
Allow: /admin/public/
"#;
        let rules = RobotsRules::parse(content);
        assert!(!rules.is_allowed("*", "/admin/dashboard"));
        assert!(!rules.is_allowed("*", "/private/data"));
        assert!(rules.is_allowed("*", "/admin/public/page"));
        assert!(rules.is_allowed("*", "/blog/post-1"));
    }

    #[test]
    fn parses_specific_user_agent_rules() {
        let content = r#"
User-agent: CrawlingoBot
Disallow: /secret/

User-agent: *
Disallow: /all/
"#;
        let rules = RobotsRules::parse(content);
        assert!(!rules.is_allowed("CrawlingoBot", "/secret/doc"));
        assert!(rules.is_allowed("CrawlingoBot", "/all/doc"));
        assert!(!rules.is_allowed("OtherBot", "/all/doc"));
    }

    #[test]
    fn parses_crawl_delay_and_sitemaps() {
        let content = r#"
User-agent: *
Crawl-delay: 2.5
Sitemap: https://example.com/sitemap.xml
"#;
        let rules = RobotsRules::parse(content);
        assert_eq!(rules.crawl_delay("*"), Some(2.5));
        assert_eq!(rules.sitemaps, vec!["https://example.com/sitemap.xml"]);
    }
}
