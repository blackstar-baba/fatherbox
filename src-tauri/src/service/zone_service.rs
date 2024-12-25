pub fn is_allow_zone(zone: &str) -> bool {
    return match zone {
        "" | "markdown" | "server" => true,
        _ => false,
    };
}

pub fn list_zones() -> Vec<String> {
    return vec![
        "markdown".to_string(),
        "server".to_string(),
        "other".to_string(),
    ];
}
