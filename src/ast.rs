use crate::{errors::EvalError, State};
#[derive(Debug, Default)]
pub enum Value {
    Number(f32),
    #[default]
    None,
}
pub enum EvalAdapter<'a> {
    Expression(&'a dyn ExprNode),
}
pub enum TreeNode {
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
