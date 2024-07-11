use crate::{
    ast::{ExprHandler, LiteralHandler},
    errors::ParseError,
    lexer::{ParseAdapter, TokenGroup, TokenStream},
    State,
};

pub trait LitParser: TokenGroup + 'static {
    fn parse_lit(&self) -> LiteralHandler;
}
pub trait ExprParser: TokenGroup + 'static {
    fn parse_expr(
        &self,
        lhs: Option<ExprHandler>,
        tokens: &mut TokenStream<'_>,
        state: &mut State,
    ) -> Result<ExprHandler, ParseError>;
}

pub struct Parser<'src> {
    tokens: TokenStream<'src>,
    nodes: Vec<ExprHandler>,
}

impl<'src> Parser<'src> {
    pub fn new(tokens: TokenStream<'src>) -> Self {
        Self {
            tokens,
            nodes: vec![],
        }
    }
    pub fn parse(&mut self, state: &mut State) -> Result<(), ParseError> {
        loop {
            let next = match self.tokens.next() {
                Some(Ok(tok)) => tok,
                Some(Err(err)) => return Err(ParseError(err.to_string())),
                None => break,
            };
            let expr = match next.adapter() {
                ParseAdapter::Literal(l) => l.parse_lit().to_expr_node(),
                ParseAdapter::Expression(e) => {
                    e.parse_expr(self.nodes.pop(), &mut self.tokens, state)?
                }
                _ => panic!(),
            };
            self.nodes.push(expr);
        }
        Ok(())
    }
    pub fn nodes(self) -> Vec<ExprHandler> {
        self.nodes
    }
}
