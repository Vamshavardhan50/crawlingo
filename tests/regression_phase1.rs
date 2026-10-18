#[cfg(not(feature = "browser"))]
use crawlingo::engine::fetcher::{BrowserFetcher, FetcherTier, Transport};
use crawlingo::engine::session::Session;
use crawlingo::extraction::{ExtractionEngine, ExtractionRule, ExtractionType};
use crawlingo::selector::SelectorQuery;
use crawlingo::util::url::UrlValidator;
use std::sync::Arc;

#[test]
fn test_regression_attribute_extraction_from_dom() {
    use std::collections::HashMap;
    let html = r#"
        <html>
            <body>
                <a class="main" href="https://crawlingo.dev/docs" title="Documentation">Read Docs</a>
                <img id="logo" src="/assets/logo.png" alt="Crawlingo Logo">
            </body>
        </html>
    "#;

    let page = crawlingo::parser::streaming::HtmlParser::parse(
        crawlingo::engine::fetcher::NormalizedResponse {
            url: "https://crawlingo.dev".to_string(),
            status: 200,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: html.into(),
            content_type: "text/html".to_string(),
            encoding: "utf-8".to_string(),
            timings: Default::default(),
        },
    )
    .unwrap();

    let rules = vec![
        ExtractionRule {
            name: "link_href".to_string(),
            selector: "a.main".to_string(),
            selector_type: "css".to_string(),
            extract_type: ExtractionType::Attribute("href".to_string()),
            default: None,
            #[cfg(feature = "python")]
            transform: None,
        },
        ExtractionRule {
            name: "img_src".to_string(),
            selector: "#logo".to_string(),
            selector_type: "css".to_string(),
            extract_type: ExtractionType::Attribute("src".to_string()),
            default: None,
            #[cfg(feature = "python")]
            transform: None,
        },
    ];

    let results = ExtractionEngine::extract(&page, &rules).unwrap();
    assert_eq!(
        results.get("link_href").unwrap(),
        "https://crawlingo.dev/docs",
        "attr:href must return DOM attribute value, not link text"
    );
    assert_eq!(
        results.get("img_src").unwrap(),
        "/assets/logo.png",
        "attr:src must return DOM attribute value"
    );
}

#[test]
fn test_regression_selector_query_centralized_dispatch() {
    let q1 = SelectorQuery::from_type_str("xpath", "//div");
    assert!(matches!(q1, SelectorQuery::XPath("//div")));

    let q2 = SelectorQuery::from_type_str("regex", r"\d+");
    assert!(matches!(q2, SelectorQuery::Regex(r"\d+")));

    let q3 = SelectorQuery::from_type_str("text", "Login");
    assert!(matches!(q3, SelectorQuery::TextAnchor("Login")));

    let q4 = SelectorQuery::from_type_str("after_text", "Price:");
    assert!(matches!(q4, SelectorQuery::AfterText("Price:")));

    let q5 = SelectorQuery::from_type_str("before_text", "USD");
    assert!(matches!(q5, SelectorQuery::BeforeText("USD")));

    let q6 = SelectorQuery::from_type_str("css", "h1.title");
    assert!(matches!(q6, SelectorQuery::Css("h1.title")));

    let q7 = SelectorQuery::from_type_str("unknown_type", ".box");
    assert!(matches!(q7, SelectorQuery::Css(".box")));
}

#[test]
fn test_regression_session_proxy_auth_credential_injection() {
    let session = Session::new();
    session.set_proxy("http://proxy.internal:8080".to_string());
    session.set_proxy_auth("alice".to_string(), "p@ssword#123".to_string());

    let proxy_with_auth = session.get_next_proxy_with_auth().unwrap();
    assert_eq!(
        proxy_with_auth, "http://alice:p%40ssword%23123@proxy.internal:8080",
        "Proxy credentials must be percent-encoded and injected into the authority"
    );
}

#[cfg(not(feature = "browser"))]
#[tokio::test]
async fn test_regression_browser_fetcher_explicit_not_supported_error() {
    let fetcher = BrowserFetcher::new(Default::default());
    let req = crawlingo::engine::fetcher::FetchRequest {
        url: "https://example.com".to_string(),
        tier: FetcherTier::Browser,
        browser_profile: Some("chrome".to_string()),
        headers: Default::default(),
        cookies: Default::default(),
        proxy: None,
        timeout: std::time::Duration::from_secs(10),
        retries: 0,
    };

    let result = fetcher.fetch(&req).await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Browser rendering"),
        "BrowserFetcher must return an explicit NotSupported error"
    );
}

#[test]
fn test_regression_session_clone_independence() {
    let parent = Session::new();
    parent.set_proxy("http://proxy1.example.com:8080".to_string());

    let clone = parent.clone_session();
    assert_eq!(
        clone.get_next_proxy().unwrap(),
        "http://proxy1.example.com:8080"
    );

    // Modifying clone's proxy should not affect parent
    clone.set_proxy("http://proxy2.example.com:8080".to_string());
    assert_eq!(
        parent.get_next_proxy().unwrap(),
        "http://proxy1.example.com:8080"
    );
    assert_eq!(
        clone.get_next_proxy().unwrap(),
        "http://proxy2.example.com:8080"
    );

    // FetchManager is independent
    let parent_mgr = parent.fetch_manager();
    let clone_mgr = clone.fetch_manager();
    assert!(
        !Arc::ptr_eq(&parent_mgr, &clone_mgr),
        "Cloned session must have an independent FetchManager instance"
    );
}

#[test]
fn test_regression_url_validator_ssrf_protection() {
    let validator = UrlValidator::default();

    // Allowed schemes
    assert!(validator.validate("http://example.com").is_ok());
    assert!(validator.validate("https://example.com/api").is_ok());

    // Rejects forbidden schemes
    assert!(validator.validate("file:///etc/passwd").is_err());
    assert!(validator.validate("ftp://mirror.example.com").is_err());
    assert!(validator.validate("javascript:void(0)").is_err());
    assert!(validator.validate("gopher://old.example.com").is_err());

    // Rejects missing host
    assert!(validator.validate("http://").is_err());
}
