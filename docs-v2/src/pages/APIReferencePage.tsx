import { motion } from 'framer-motion';
import CodeBlock from '@/components/ui/CodeBlock';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const apiSections = [
  {
    title: 'Session',
    id: 'session',
    description: 'Central configuration for all scraping operations.',
    methods: [
      { name: 'Session()', desc: 'Create a new session with defaults.', code: 'session = Session()' },
      { name: '.headers(dict)', desc: 'Set custom HTTP headers.', code: 'session.headers({"Accept-Language": "en-US"})', returns: 'Session' },
      { name: '.cookies(dict)', desc: 'Set cookies for requests.', code: 'session.cookies({"sid": "abc"})', returns: 'Session' },
      { name: '.proxy(str)', desc: 'Set proxy URL.', code: 'session.proxy("http://proxy:8080")', returns: 'Session' },
      { name: '.rate_limit(float)', desc: 'Requests per second per host.', code: 'session.rate_limit(5.0)', returns: 'Session' },
      { name: '.timeout(int)', desc: 'Request timeout in seconds.', code: 'session.timeout(30)', returns: 'Session' },
      { name: '.auto_match(bool)', desc: 'Enable self-healing selectors.', code: 'session.auto_match(True)', returns: 'Session' },
      { name: '.fetcher_tier(str)', desc: '"standard" or "stealthy" (browser impersonation).', code: 'session.fetcher_tier("stealthy")', returns: 'Session' },
      { name: '.browser_profile(str)', desc: '"chrome", "firefox", or "safari".', code: 'session.browser_profile("chrome")', returns: 'Session' },
      { name: '.fingerprint_path(str)', desc: 'Path for auto-match fingerprint store.', code: 'session.fingerprint_path(".crawlingo")', returns: 'Session' },
      { name: '.auto_match_weights(dict)', desc: 'Set custom similarity scoring weights for healing.', code: 'session.auto_match_weights({"text": 2.0, "class": 1.0})', returns: 'Session' },
      { name: '.proxy_pool(list)', desc: 'Rotate requests through a pool of proxies round-robin.', code: 'session.proxy_pool(["http://p1:80", "http://p2:80"])', returns: 'Session' },
      { name: '.proxy_provider(str)', desc: 'Pull rotating proxies dynamically from a provider API URL.', code: 'session.proxy_provider("https://provider.api/proxies")', returns: 'Session' },
      { name: '.page(url)', desc: 'Fetch a single page.', code: 'page = session.page("https://example.com")', returns: 'Page' },
      { name: '.dataset(url)', desc: 'Create a dataset builder.', code: 'ds = session.dataset("https://example.com")', returns: 'Dataset' },
      { name: '.crawl(url)', desc: 'Create a multi-page crawler.', code: 'cr = session.crawl("https://example.com")', returns: 'Crawl' },
      { name: '.watch(url)', desc: 'Create a change monitor.', code: 'w = session.watch("https://example.com")', returns: 'Watch' },
    ],
  },
  {
    title: 'Page',
    id: 'page',
    description: 'Lazy-loaded page with selector methods. Fetches on first access.',
    methods: [
      { name: '.url', desc: 'Final URL after redirects.', returns: 'str' },
      { name: '.status', desc: 'HTTP status code.', returns: 'int' },
      { name: '.html()', desc: 'Raw HTML content.', returns: 'str' },
      { name: '.title()', desc: 'Content of <title> tag.', returns: 'str' },
      { name: '.css(selector)', desc: 'Query by CSS selector.', code: 'page.css("h1")', returns: 'ElementCollection' },
      { name: '.xpath(query)', desc: 'Query by XPath expression.', code: 'page.xpath("//div[@class=\'x\']")', returns: 'ElementCollection' },
      { name: '.find_text(text)', desc: 'Find nodes containing text.', code: 'page.find_text("Price:")', returns: 'ElementCollection' },
      { name: '.after_text(text)', desc: 'Element after text anchor.', code: 'page.after_text("Price:")', returns: 'ElementCollection' },
      { name: '.before_text(text)', desc: 'Element before text anchor.', code: 'page.before_text("Footer")', returns: 'ElementCollection' },
      { name: '.regex(pattern)', desc: 'Match text against regex.', code: 'page.regex(r"\\d+\\.\\d{2}")', returns: 'ElementCollection' },
    ],
  },
  {
    title: 'Element',
    id: 'element',
    description: 'Single DOM element with navigation and attribute access.',
    methods: [
      { name: '.text()', desc: 'Text content of element.', returns: 'str' },
      { name: '.html()', desc: 'Inner HTML.', returns: 'str' },
      { name: '.attr(name)', desc: 'Get attribute value.', code: 'el.attr("href")', returns: 'str' },
      { name: '.attrs()', desc: 'All attributes as dict.', returns: 'dict' },
      { name: '.parent()', desc: 'Parent element.', returns: 'Element | None' },
      { name: '.children()', desc: 'Child elements.', returns: 'ElementCollection' },
      { name: '.next()', desc: 'Next sibling.', returns: 'Element | None' },
      { name: '.prev()', desc: 'Previous sibling.', returns: 'Element | None' },
      { name: '.siblings()', desc: 'All siblings.', returns: 'ElementCollection' },
    ],
  },
  {
    title: 'ElementCollection',
    id: 'elementcollection',
    description: 'Collection of elements with chainable methods.',
    methods: [
      { name: '.text()', desc: 'Concatenated text of all elements.', returns: 'str' },
      { name: '.texts()', desc: 'List of text values.', returns: 'List[str]' },
      { name: '.attr(name)', desc: 'Attribute from first element.', returns: 'str' },
      { name: '.first()', desc: 'First element.', returns: 'Element | None' },
      { name: '.last()', desc: 'Last element.', returns: 'Element | None' },
      { name: '.nth(n)', desc: 'Element at index n.', returns: 'Element | None' },
      { name: '.filter(fn)', desc: 'Filter elements.', code: 'coll.filter(lambda e: "sale" in e.text())', returns: 'ElementCollection' },
      { name: '.map(fn)', desc: 'Map elements to values.', code: 'coll.map(lambda e: e.text())', returns: 'list' },
    ],
  },
  {
    title: 'Dataset',
    id: 'dataset',
    description: 'Structured data extraction from a single URL.',
    methods: [
      { name: '.field(name, selector, selector_type, default)', desc: 'Add extraction field.', code: 'ds.field("title", "h1", selector_type="css")', returns: 'Dataset' },
      { name: '.auto_match(bool)', desc: 'Enable auto-match for this dataset.', returns: 'Dataset' },
      { name: '.build()', desc: 'Execute extraction (sync).', returns: 'DatasetResult' },
    ],
  },
  {
    title: 'DatasetResult',
    id: 'datasetresult',
    description: 'Result of a dataset build with export methods.',
    methods: [
      { name: '[key]', desc: 'Access field by name.', code: 'result["title"]', returns: 'str' },
      { name: '.to_json(path)', desc: 'Export to JSON file.', returns: 'None' },
      { name: '.to_csv(path)', desc: 'Export to CSV file.', returns: 'None' },
      { name: '.to_parquet(path)', desc: 'Export to Parquet file.', returns: 'None' },
      { name: '.df()', desc: 'Convert to Pandas DataFrame.', returns: 'DataFrame' },
    ],
  },
  {
    title: 'Crawl',
    id: 'crawl',
    description: 'Recursive crawler with concurrency limits, delay pacing, webhooks, and background scheduling.',
    methods: [
      { name: 'Crawl(start_url, session)', desc: 'Create a new crawler.', code: 'crawl = Crawl("https://example.com", session)', returns: 'Crawl' },
      { name: '.follow(selector)', desc: 'CSS selector of links to follow recursively.', code: 'crawl.follow("a.next-page")', returns: 'Crawl' },
      { name: '.limit(pages)', desc: 'Limit the crawler to a maximum page count.', code: 'crawl.limit(50)', returns: 'Crawl' },
      { name: '.depth(max_depth)', desc: 'Maximum links hop depth.', code: 'crawl.depth(3)', returns: 'Crawl' },
      { name: '.concurrency(n)', desc: 'Max concurrent fetching workers.', code: 'crawl.concurrency(4)', returns: 'Crawl' },
      { name: '.delay(seconds)', desc: 'Politeness pacing delay.', code: 'crawl.delay(0.5)', returns: 'Crawl' },
      { name: '.field(name, selector, selector_type, default)', desc: 'Define fields to extract from every page.', code: 'crawl.field("title", "h1", selector_type="css")', returns: 'Crawl' },
      { name: '.webhook(url)', desc: 'Real-time JSON result delivery webhook URL.', code: 'crawl.webhook("https://api.myweb.com/webhook")', returns: 'Crawl' },
      { name: '.schedule(interval_seconds)', desc: 'Run scheduled recurring crawl background loops.', code: 'crawl.schedule(3600)', returns: 'None' },
      { name: '.build()', desc: 'Run crawl loops synchronously.', code: 'results = crawl.build()', returns: 'CrawlResults' },
    ],
  },
];

export default function APIReferencePage() {
  return (
    <div className="space-y-12">
      {/* Title */}
      <motion.div
        className="pb-6 border-b border-gray-200 dark:border-white/10"
        initial={{ opacity: 0, y: 15 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <h1 className="text-display-sm md:text-display-md font-bold text-black dark:text-white tracking-tight mb-3">
          API References
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Detailed schemas, function signatures, types, default values, and usage specifications for Crawlingo classes.
        </p>
      </motion.div>

      {/* Sections */}
      <div className="space-y-12">
        {apiSections.map((section, i) => (
          <section key={i} id={section.id} className="scroll-mt-20">
            <h2 className="text-lg font-bold text-black dark:text-white mb-2">{section.title}</h2>
            <p className="text-xs text-gray-500 dark:text-gray-400 mb-6">{section.description}</p>

            <div className="space-y-4">
              {section.methods.map((method, j) => (
                <HoverBorderCard key={j} className="p-5">
                  <div className="flex items-start justify-between gap-4">
                    <div>
                      <span className="font-mono text-xs text-black dark:text-white font-semibold">
                        {method.name}
                      </span>
                      <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">{method.desc}</p>
                    </div>
                    {method.returns && (
                      <span className="text-[10px] font-mono text-gray-500 bg-gray-50 dark:bg-white/5 border border-gray-200 dark:border-white/10 px-2 py-0.5 rounded shrink-0">
                        {method.returns}
                      </span>
                    )}
                  </div>
                  {method.code && (
                    <div className="mt-3">
                      <CodeBlock language="python" code={method.code} className="my-0" />
                    </div>
                  )}
                </HoverBorderCard>
              ))}
            </div>
          </section>
        ))}
      </div>
    </div>
  );
}
