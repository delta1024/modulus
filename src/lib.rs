use std::{error, fmt, iter::Peekable, ops::Deref};

use lexer::Lexer;

pub mod evaluator;
pub mod lexer;
pub mod plugins;
pub mod value;
pub use evaluator::Evaluator;
use value::Value;

#[derive(Debug)]
pub enum LanguageLevel {
    Declaration,
    Statement,
    Expression,
    Literal,
}
pub trait TokenGroup: fmt::Debug {
    fn line(&self) -> u32;
    fn lexum(&self) -> &str;
    fn lang_level(&self) -> LanguageLevel;
    fn lit_handler(&self) -> Option<&dyn LitParser> {
        None
    }
    fn expr_handler(&self) -> Option<&dyn ExperParser> {
        None
    }
}
pub type ParseScanner<'src> = Peekable<Lexer<'src>>;
pub type ExprHandler = Box<dyn ExprPlugin>;

pub trait LitParser: TokenGroup {
    fn parse_lit(&self) -> Box<(dyn TreeNode + 'static)>;
}

pub trait ExperParser: TokenGroup {
    fn parse_expr<'src>(
        &self,
        scanner: &mut ParseScanner<'src>,
        lhs: Option<Box<(dyn ExprPlugin + 'static)>>,
    ) -> Box<(dyn TreeNode + 'static)>;
}

#[derive(Debug)]
pub enum EvalError{
    Expr(Value, Box<dyn error::Error>),
}
pub trait ExprPlugin: TreeNode +  'static + fmt::Debug {
    fn evaluate(&self) -> Result<Value, EvalError>;
}
pub trait LitPlugin: TreeNode + 'static + fmt::Debug {
    fn value(&self) -> Value;
}
impl<T: LitPlugin> ExprPlugin for T {
    fn evaluate(&self) -> Result<Value, EvalError> {
        Ok(self.value())
    }
}
pub trait TreeNode:  fmt::Debug + 'static {
    fn as_lit(&self) -> Option<&dyn LitPlugin> {
        None
    }
    fn as_expr(&self) -> Option<&dyn ExprPlugin> {
        None
    }
}

#[repr(transparent)]
pub struct AstNode(Box<dyn TreeNode>);

impl Deref for AstNode {
    type Target = dyn TreeNode;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
