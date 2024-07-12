use core::fmt;

use crate::{errors::EvalError, State};
#[derive(Debug, Default)]
pub enum Value {
    Number(f32),
    #[default]
    None,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::None => write!(f, "none"),
        }
    }
}
pub enum TreeNode {
    Declaration(Box<dyn DeclNode>),
    Statement(Box<dyn StmtNode>),
    Expression(Box<dyn ExprNode>),
}

pub type LiteralHandler = Box<dyn LiteralNode>;
pub trait LiteralNode: ExprNode {
    fn eval_literal(&self) -> Value;
}
pub trait ExprNode {
    fn to_expr_node(self: Box<Self>) -> Box<dyn ExprNode>;
    fn eval_expr(&self, state: &mut State) -> Result<Value, EvalError>;
}
impl<T: LiteralNode + 'static> ExprNode for T {
    fn to_expr_node(self: Box<Self>) -> Box<dyn ExprNode> {
        self
    }
    fn eval_expr(&self, _: &mut State) -> Result<Value, EvalError> {
        Ok(self.eval_literal())
    }
}
impl From<Box<dyn ExprNode>> for TreeNode {
    fn from(value: Box<dyn ExprNode>) -> Self {
        Self::Expression(value)
    }
}
pub type ExprHandler = Box<dyn ExprNode>;

pub trait StmtNode {
    fn to_stmt_node(self: Box<Self>) -> Box<dyn StmtNode>;
    fn eval_stmt(&self, state: &mut State) -> Result<(), EvalError>;
}

impl From<Box<dyn StmtNode>> for TreeNode {
    fn from(value: Box<dyn StmtNode>) -> Self {
        Self::Statement(value)
    }
}

pub trait DeclNode {
    fn to_decl_node(self: Box<Self>) -> Box<dyn StmtNode>;
    fn eval_decl(&self, state: &mut State) -> Result<(), EvalError>;
}
