use crate::{
    analyzers::{self, Analyzer},
    tokenizers::{self, Tokenizer},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchEngine {
    tokenizer: Tokenizer,
    analyzers: Vec<Analyzer>,
    inverted_index: HashMap<String, HashMap<String, u32>>,
    documents: HashMap<String, String>,
}

impl SearchEngine {
    pub fn index(&mut self, key: String, content: String) {
        self.documents.insert(key.clone(), content.clone());
        for token in (self.tokenizer)(content) {
            let token = self.analyze(token);
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
    fn analyze(&self, mut token: String) -> String {
        for analyzer in &self.analyzers {
            token = analyzer(token);
        }
        token
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        SearchEngine {
            tokenizer: tokenizers::whitespace(),
            analyzers: vec![analyzers::lowercase(), analyzers::alphanumeric()],
            inverted_index: Default::default(),
            documents: Default::default(),
        }
    }
}
