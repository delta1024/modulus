pub mod sub_state;
use std::{collections::HashMap, rc::Rc};


pub use sub_state::*;

use crate::ast::Value;

#[derive(Default)]
pub struct State{
    pub sub_states: HashMap<String, Rc<dyn SubStateExt>>,
    pub stack: Vec<Value>,
}

impl State {
    pub fn new() -> Self{
        Self::default()
    }
    pub fn add_sub_state(&mut self, state: impl SubStateExt) {
        self.sub_states.insert(state.state_name().to_string(), Rc::new(state));
    }
    pub fn get_get_state(&mut self, state_name: &str) -> Option<Rc<dyn SubStateExt>> {
        self.sub_states.get(state_name).map(Rc::clone)
    }
    pub fn remove_sub_state(&mut self, state_name: &str) {
        self.sub_states.remove(state_name);
    }
    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }
    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}


