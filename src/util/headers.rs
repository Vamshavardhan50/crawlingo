use std::collections::HashMap;

/// Case-insensitive header lookup from a HashMap.
pub fn get_header<'a>(map: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    let lower_key = key.to_lowercase();
    for (k, v) in map {
        if k.to_lowercase() == lower_key {
            return Some(v.as_str());
        }
    }
    None
}
