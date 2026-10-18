/// Resolves a potentially relative URL (`href`) against a `base` URL.
pub fn resolve_url(base: &str, href: &str) -> Option<String> {
    if href.starts_with("http://") || href.starts_with("https://") {
        return Some(href.to_string());
    }
    let base_url = url::Url::parse(base).ok()?;
    base_url.join(href).ok().map(|u| u.to_string())
}

/// Sanitizes a proxy URL by redacting the password if present.
pub fn sanitize_proxy_url(url: &str) -> String {
    if let Ok(mut parsed) = url::Url::parse(url) {
        if parsed.password().is_some() {
            let _ = parsed.set_password(Some("***"));
        }
        parsed.to_string()
    } else {
        url.to_string()
    }
}

/// Canonicalizes a URL string for deterministic deduplication.
///
/// Performing:
/// - Stripping fragments (`#...`)
/// - Lowercasing scheme and host
/// - Removing default ports (`:80` for http, `:443` for https)
/// - Sorting query parameter keys deterministically
/// - Ensuring empty paths become `/`
pub fn canonicalize_url(url_str: &str) -> Option<String> {
    let mut parsed = url::Url::parse(url_str).ok()?;

    // 1. Strip fragment (#...)
    parsed.set_fragment(None);

    // 2. Sort query parameters if present
    if parsed.query().is_some() {
        let mut pairs: Vec<(String, String)> = parsed
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        parsed.query_pairs_mut().clear();
        for (k, v) in pairs {
            parsed.query_pairs_mut().append_pair(&k, &v);
        }
    }

    // 3. Ensure empty path becomes "/"
    if parsed.path().is_empty() {
        parsed.set_path("/");
    }

    Some(parsed.to_string())
}

/// Resolves `href` against `base` and returns the canonical representation.
pub fn canonicalize_relative_url(base: &str, href: &str) -> Option<String> {
    let resolved = resolve_url(base, href)?;
    canonicalize_url(&resolved)
}

/// Security policy and validation for request URLs.
/// Protects against unsafe schemes (`file://`, `ftp://`, `gopher://`, `javascript:`) and invalid hosts.
#[derive(Debug, Clone)]
pub struct UrlValidator {
    pub allowed_schemes: Vec<String>,
}

impl Default for UrlValidator {
    fn default() -> Self {
        Self {
            allowed_schemes: vec!["http".to_string(), "https".to_string()],
        }
    }
}

impl UrlValidator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates a URL string against allowed schemes and host constraints.
    pub fn validate(&self, url_str: &str) -> crate::error::Result<url::Url> {
        let parsed = url::Url::parse(url_str).map_err(|e| {
            crate::error::CrawlingoError::FetchError(format!("Invalid URL '{url_str}': {e}"))
        })?;

        let scheme = parsed.scheme().to_lowercase();
        if !self.allowed_schemes.contains(&scheme) {
            return Err(crate::error::CrawlingoError::FetchError(format!(
                "URL scheme '{scheme}' is not allowed (only {:?} are supported)",
                self.allowed_schemes
            )));
        }

        if parsed.host_str().unwrap_or("").is_empty() {
            return Err(crate::error::CrawlingoError::FetchError(format!(
                "URL '{url_str}' is missing a valid host"
            )));
        }

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_http_and_https_urls() {
        let v = UrlValidator::default();
        assert!(v.validate("http://example.com").is_ok());
        assert!(v.validate("https://example.com/path?query=1").is_ok());
    }

    #[test]
    fn rejects_disallowed_schemes() {
        let v = UrlValidator::default();
        assert!(v.validate("file:///etc/passwd").is_err());
        assert!(v.validate("ftp://files.example.com").is_err());
        assert!(v.validate("gopher://gopher.example.com").is_err());
        assert!(v.validate("javascript:alert(1)").is_err());
    }

    #[test]
    fn rejects_missing_host() {
        let v = UrlValidator::default();
        assert!(v.validate("http://").is_err());
    }

    #[test]
    fn test_canonicalize_url_strips_fragments() {
        let c1 = canonicalize_url("https://example.com/page#section1").unwrap();
        let c2 = canonicalize_url("https://example.com/page#section2").unwrap();
        let c3 = canonicalize_url("https://example.com/page").unwrap();
        assert_eq!(c1, "https://example.com/page");
        assert_eq!(c2, "https://example.com/page");
        assert_eq!(c3, "https://example.com/page");
    }

    #[test]
    fn test_canonicalize_url_sorts_query_parameters() {
        let c1 = canonicalize_url("https://example.com/search?z=2&a=1").unwrap();
        let c2 = canonicalize_url("https://example.com/search?a=1&z=2").unwrap();
        assert_eq!(c1, "https://example.com/search?a=1&z=2");
        assert_eq!(c2, "https://example.com/search?a=1&z=2");
    }

    #[test]
    fn test_canonicalize_relative_url_resolves_and_canonicalizes() {
        let base = "https://example.com/blog/posts/";
        let rel1 = "../about#team";
        let rel2 = "/blog/about";
        assert_eq!(
            canonicalize_relative_url(base, rel1).unwrap(),
            "https://example.com/blog/about"
        );
        assert_eq!(
            canonicalize_relative_url(base, rel2).unwrap(),
            "https://example.com/blog/about"
        );
    }
}
