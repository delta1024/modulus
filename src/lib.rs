use std::{fmt, iter::Peekable, str::CharIndices};

pub mod lexer_plugins;
pub trait TokenGroup<'src>: fmt::Debug + 'src {
    fn line(&self) -> u32;
    fn lexum(&self) -> &'src str;
}
pub type LexerPos<'src> = Peekable<CharIndices<'src>>;
pub type LexerHandler<'src> = Box<dyn LexerPlugin<'src>>;
pub enum LexerError {
    InvalidToken(char),
    IncompleteToken(Box<dyn std::error::Error>),
}
pub struct Lexer<'src>
where
    Self: 'src,
    {
    source: &'src str,
    line: u32,
    position: LexerPos<'src>,
    handlers: Vec<LexerHandler<'src>>,
}
impl<'src> Lexer<'src> {
    pub fn builder(source: &'src str) -> LexerBuilder<'src> {
        LexerBuilder {
            source,
            handlers: vec![],
        }
    }
}
impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Box<dyn TokenGroup<'src>>,LexerError>;
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
    handlers: Vec<LexerHandler<'src>>,
}
impl<'src> LexerBuilder<'src> {
    pub fn add_handler(&mut self, handler: impl LexerPlugin<'src>) -> &mut Self {
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

pub trait LexerPlugin<'src>:  'src {
    fn is_handler(&self, c: char) -> bool;
    fn handel_lexum(&self, source: &'src str, cur_pos: (usize, char),line: u32, pos: &mut LexerPos<'src>) -> Result<Box<dyn TokenGroup<'src>>, LexerError>;
}
