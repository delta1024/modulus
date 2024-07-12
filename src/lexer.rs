use std::{fmt, iter::Peekable, rc::Rc, str::CharIndices};

use crate::{
    errors::LexerError,
    parser::{ExprParser, LitParser},
};
pub enum ParseAdapter {
    Declaration,
    Statement,
    Expression(Box<dyn ExprParser>),
    Literal(Box<dyn LitParser>),
}

pub trait TokenGroup: fmt::Debug + 'static {
    fn to_token(self: Box<Self>) -> Box<dyn TokenGroup>;
    fn adapter(self: Box<Self>) -> ParseAdapter;
    fn lexum(&self) -> &str;
    fn line(&self) -> u32;
}

pub type TokenHandler = Box<dyn TokenGroup>;

pub type CharStream<'src> = Peekable<CharIndices<'src>>;
pub trait LexerPlugin: 'static {
    fn as_lexer(self: Rc<Self>) -> Rc<dyn LexerPlugin>;
    fn handles_char(&self, c: char) -> bool;
    fn lex_token<'src>(
        &self,
        source: &'src str,
        pos: (usize, char),
        line: u32,
        scanner: &mut CharStream<'src>,
    ) -> Result<TokenHandler, LexerError>;
}
pub type LexHandler = Rc<dyn LexerPlugin>;
pub struct Lexer<'src> {
    source: &'src str,
    scanner: CharStream<'src>,
    plugins: Vec<LexHandler>,
    line: u32,
}

impl<'src> Lexer<'src> {
    pub fn builder() -> LexerBuilder<'src> {
        LexerBuilder::default()
    }
}
impl Iterator for Lexer<'_> {
    type Item = Result<TokenHandler, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        let Some((pos, c)) = self.scanner.next() else {
            return None;
        };
        for handler in &self.plugins {
            if handler.handles_char(c) {
                return match handler.lex_token(self.source, (pos, c), self.line, &mut self.scanner)
                {
                    Ok(token) => Some(token).map(Ok),
                    Err(err) => Some(err).map(Err),
                };
            }
        }
        Some(LexerError::UnknownToken(c)).map(Err)
    }
}
pub type TokenStream<'src> = Peekable<Lexer<'src>>;
#[derive(Default)]
pub struct LexerBuilder<'src> {
    source: Option<&'src str>,
    plugins: Vec<LexHandler>,
}
impl<'src> LexerBuilder<'src>
where
Self: 'src,
{
    pub fn source(&mut self, source: &'src str) -> &mut Self {
        self.source = Some(source);
        self
    }
    pub fn plugin<T: Sized + LexerPlugin + Default>(&mut self) -> &mut Self {
        self.plugins.push(Rc::new(T::default()));
        self
    }
    pub fn plugins<'a, 'b>(&'a mut self, plugins: Vec<&Rc<dyn LexerPlugin>>) -> &'a mut Self {
        for handler in plugins {
            self.plugins.push(Rc::clone(handler));
        }
        self
    }
    pub fn build(&mut self) -> Lexer<'src> {
        Lexer {
            source: self.source.unwrap(),
            plugins: self.plugins.drain(..).collect(),
            scanner: self.source.unwrap().char_indices().peekable(),
            line: 0,
        }
    }
}
