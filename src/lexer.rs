use std::{fmt,iter::Peekable, str::CharIndices};

use crate::TokenGroup;

pub type LexerPos<'src> = Peekable<CharIndices<'src>>;
pub type LexerHandler = Box<dyn LexerPlugin>;
#[derive(Debug)]
pub enum LexerError {
    InvalidToken(char),
    IncompleteToken(Box<dyn std::error::Error>),
}
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(c) => write!(f, "Error: Invalid token: {c}"),
            Self::IncompleteToken(err) => write!(f, "Error: {err}"),
        }
    }
}
pub struct Lexer<'src>
where
    Self: 'src,
    {
    source: &'src str,
    line: u32,
    position: LexerPos<'src>,
    handlers: Vec<LexerHandler>,
}
impl<'src> Lexer<'src> {
    pub fn builder(source: &'src str) -> LexerBuilder<'src> {
        LexerBuilder {
            source,
            handlers: vec![],
        }
    }
}
impl Iterator for Lexer<'_> {
    type Item = Result<Box<dyn TokenGroup>,LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.position.peek().is_some_and(|(_,c)| match c {
            '\t' | '\r' | ' ' => true,
            '\n' => {
                self.line += 1;
                true
            },
            _ => false,
        }) {
            self.position.next();
        }
        match self.position.next() {
            Some((pos, c)) => {
                for handler in &self.handlers {
                    if handler.is_handler(c) {
                        return Some(handler.handel_lexum(self.source, (pos,c),self.line, &mut self.position));
                    }
                }
                Some(Err(LexerError::InvalidToken(c)))
            }
            None => None,
        }
    }
}
pub struct LexerBuilder<'src> {
    source: &'src str,
    handlers: Vec<LexerHandler>,
}
impl<'src> LexerBuilder<'src> {
    pub fn add_handler(&mut self, handler: impl LexerPlugin) -> &mut Self {
        self.handlers.push(Box::new(handler));
        self
    }
    pub fn build(&mut self) -> Lexer<'src> {
        Lexer {
            source: self.source,
            line: 0,
            position: self.source.char_indices().peekable(),
            handlers: self.handlers.drain(..).collect(),
        }
    }
}

pub trait LexerPlugin: 'static {
    fn is_handler(&self, c: char) -> bool;
    fn handel_lexum<'src>(&self, source: &'src str, cur_pos: (usize, char),line: u32, pos: &mut LexerPos<'src>) -> Result<Box<dyn TokenGroup>, LexerError>;
}
