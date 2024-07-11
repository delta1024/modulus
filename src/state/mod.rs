pub mod sub_state;
use std::{collections::HashMap, rc::Rc};

pub use sub_state::*;

#[derive(Default)]
pub struct State{
    sub_states: HashMap<String, Rc<dyn SubStateExe>>,
}

impl State {
    pub fn new() -> Self{
        Self::default()
    }
    pub fn add_state(&mut self, state: impl SubStateExe) {
        self.sub_states.insert(state.state_name().to_string(), Rc::new(state));
    }

}


