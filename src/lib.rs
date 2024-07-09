use std::{fmt, iter::Peekable, ops::Deref};

use lexer::Lexer;

pub mod evaluator;
pub mod lexer;
pub mod plugins;
pub use evaluator::Evaluator;

#[derive(Debug)]
pub enum LanguageLevel {
    Declaration,
    Statement,
    Expression,
}
pub trait TokenGroup: fmt::Debug {
    fn line(&self) -> u32;
    fn lexum(&self) -> &str;
    fn lang_level(&self) -> LanguageLevel;
    fn expr_handler(&self) -> Option<&dyn ExperParser> {
        None
    }
}
pub type ParseScanner<'src> = Peekable<Lexer<'src>>;
pub type ExprHandler = Box<dyn ExprPlugin>;

pub trait ExperParser: TokenGroup {
    fn parse_expr<'src>(
        &self,
        scanner: &mut ParseScanner<'src>,
        lhs: Option<Box<(dyn ExprPlugin + 'static)>>,
    ) -> Box<(dyn TreeNode + 'static)>;
}

pub trait ExprPlugin: TreeNode +  'static + fmt::Debug {
    fn evaluate(&self) -> Option<f32> {
        None
    }
}

pub trait TreeNode:  fmt::Debug + 'static {
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
