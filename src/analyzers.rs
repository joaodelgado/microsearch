pub type Analyzer = fn(String) -> String;

pub fn lowercase() -> Analyzer {
    |token| token.to_lowercase()
}

pub fn alphanumeric() -> Analyzer {
    |token| token.chars().filter(|c| c.is_alphanumeric()).collect()
}
