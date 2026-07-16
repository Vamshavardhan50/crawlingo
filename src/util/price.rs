/// Normalizes a price string and verifies it is a valid float.
pub fn parse_price_string(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let cleaned: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();

    let dot_count = cleaned.matches('.').count();
    let normalized = if dot_count > 1 {
        let mut parts: Vec<&str> = cleaned.split('.').collect();
        let decimal = parts.pop().unwrap_or("0");
        let integer = parts.join("");
        format!("{}.{}", integer, decimal)
    } else {
        cleaned
    };

    if normalized.parse::<f64>().is_ok() {
        Some(normalized)
    } else {
        None
    }
}
