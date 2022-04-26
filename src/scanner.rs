use crate::token::Token;

pub struct Scanner<'a> {
    #[allow(unused)]
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn scan_tokens(self) -> Vec<Token> {
        vec![]
    }
}
