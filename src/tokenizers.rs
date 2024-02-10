pub type Tokenizer = fn(String) -> Vec<String>;

pub fn whitespace() -> Tokenizer {
    |content| {
        content
            .split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect()
    }
}
