use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cache {
    store: HashMap<String, Token>,
}

impl Cache {
    pub fn new() -> Cache {
        let store = HashMap::new();

        return Cache { store };
    }

    pub fn store(&mut self, token: &Token) {
        let t = token.clone();
        self.store.insert(token.clone().id(), t);
    }

    pub fn list(self) -> HashMap<String, Token> {
        self.store
    }

    pub fn get(self, id: String) -> Option<Token> {
        match self.store.get(&id) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }
}
