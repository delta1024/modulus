use core::panic;

use crate::{ast::TreeNode, errors::{EvalError, ParseError}, lexer::{LexHandler, Lexer, LexerPlugin}, parser::Parser, State};


#[derive(Default)]
pub struct Runtime<'src> {
    global_extentions: Vec<LexHandler>,
    parser: Option<Parser<'src>>,
    pub(crate) state: State,
    nodes: Vec<TreeNode>,
}
impl<'src> Runtime<'src> {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::default()
    }
    pub fn set_source(&mut self, source: &'src str) {
        self.parser =  Some(Lexer::builder().source(source).plugins(self.global_extentions.as_slice().iter().collect()).build()).map(|l| Parser::new(l.peekable()));
    }
    pub fn parse_source(&mut self) -> Result<(), ParseError> {
        let mut parser = self.parser.take().expect("source must be set");
        parser.parse(&mut self.state)?;
        self.nodes = parser.nodes().into_iter().map(|n| n.into()).collect();
        Ok(())
    }
    pub fn eval_nodes(&mut self) -> Result<(), EvalError> {
        for node in self.nodes.drain(..) {
            let expr = match node {
                TreeNode::Expression(expr) => expr.eval_expr(&mut self.state)?,
                _ => panic!("not implemented yet"),
            };
            self.state.push(expr);
        }
        Ok(())
    }
    pub fn print_stack(&self) {
        for val in &self.state.stack {
            println!("{val}");
        }
    }
}

impl From<Vec<LexHandler>> for Runtime<'_> {
    fn from(value: Vec<LexHandler>) -> Self {
        Self { global_extentions: value, ..Self::default() }
    }
}

#[derive(Default)]
pub struct  RuntimeBuilder {
    extentions: Vec<LexHandler>,
}

impl RuntimeBuilder {
    pub fn extention(&mut self, plugin: impl LexerPlugin) -> &mut Self {
        self.extentions.push(Box::new(plugin));
    self 
}
    pub fn build<'a>(self) -> Runtime<'a> {
        Runtime {
            global_extentions: self.extentions,
            ..Default::default()
        }
    }
}
