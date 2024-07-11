use std::{any::Any, fmt, ops::Deref};

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
pub trait TreeNode: fmt::Debug + 'static {
    fn adapter(&self) -> EvalAdapter<'_>;
}

#[repr(transparent)]
pub struct TreeHandler(Box<dyn TreeNode>);
impl Deref for TreeHandler {
    type Target = dyn TreeNode;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
pub type LiteralHandler = Box<dyn LiteralNode>;
pub trait LiteralNode: TreeNode {
    fn eval_literal(&self) -> Value;
}
pub trait ExprNode: TreeNode {
    fn eval_expr(&self, state: &mut State) -> Result<Value, EvalError>;
}
impl<T: LiteralNode> ExprNode for T {
    fn eval_expr(&self, _: &mut State) -> Result<Value, EvalError> {
        Ok(self.eval_literal())
    }
}
pub type ExprHandler = Box<dyn ExprNode>;
