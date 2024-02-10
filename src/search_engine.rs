use std::collections::HashMap;

use crate::tokenizers::{self, Tokenizer};

#[derive(Debug)]
pub struct SearchEngine {
    tokenizer: Tokenizer,
    inverted_index: HashMap<String, HashMap<String, u32>>,
    documents: HashMap<String, String>,
}

impl SearchEngine {
    pub fn index(&mut self, key: String, content: String) {
        self.documents.insert(key.clone(), content.clone());
        for token in (self.tokenizer)(content) {
            self.inverted_index
                .entry(token)
                .and_modify(|token_index| {
                    token_index
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                })
                .or_insert_with(|| {
                    let mut token_index = HashMap::new();
                    token_index.insert(key.clone(), 1);
                    token_index
                });
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        SearchEngine {
            tokenizer: tokenizers::whitespace(),
            inverted_index: Default::default(),
            documents: Default::default(),
        }
    }
}
