use std::{fmt, iter::Peekable, ops::Deref, str::CharIndices};

use crate::{errors::LexerError, ExprParser, LitParser};
pub enum ParseAdapter<'a> {
    Declaration,
    Statement,
    Expression(&'a dyn ExprParser),
    Literal(&'a dyn LitParser),
}

pub trait TokenGroup: fmt::Debug + 'static {
    fn level(&self) -> ParseAdapter;
    fn lexum(&self) -> &str;
    fn line(&self) -> u32;
}

#[repr(transparent)]
pub struct TokenHandler(Box<dyn TokenGroup>);

impl TokenHandler {
    pub fn new(token: impl TokenGroup) -> Self {
        Self(Box::new(token))
    }
}

impl Deref for TokenHandler {
    type Target = dyn TokenGroup;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
pub type CharStream<'src> = Peekable<CharIndices<'src>>;
pub trait LexerPlugin: 'static {
    fn handles_char(&self, c: char) -> bool;
    fn lex_token<'src>(
        &self,
        source: &'src str,
        pos: (usize, char),
        line: u32,
        scanner: &mut CharStream<'src>,
    ) -> Result<TokenHandler, LexerError>;
}
#[repr(transparent)]
pub struct LexHandler(Box<dyn LexerPlugin>);
impl LexHandler {
    pub fn new(plugin: impl LexerPlugin) -> Self {
        Self(Box::new(plugin))
    }
}
impl Deref for LexHandler {
    type Target = dyn LexerPlugin;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
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
impl<'src> LexerBuilder<'src> {
    pub fn source(&mut self, source: &'src str) -> &mut Self {
        self.source = Some(source);
        self
    }
    pub fn plugin(&mut self, plugin: impl LexerPlugin) -> &mut Self {
        self.plugins.push(LexHandler::new(plugin));
        self
    }
    pub fn build(self) -> Lexer<'src> {
        Lexer {
            source: self.source.unwrap(),
            plugins: self.plugins,
            scanner: self.source.unwrap().char_indices().peekable(),
            line: 0,
        }
    }
}
