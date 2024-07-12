use std::{any::Any, rc::Rc};
pub trait SubState:  'static {
    fn state_name(&self) -> &'static str;
}

pub trait SubStateExt: SubState + 'static{
    fn as_state(self: Rc<Self>) -> Rc<dyn SubStateExt>;
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> ;
}

impl<T: SubState + Sized> SubStateExt for T {
    fn as_state(self: Rc<Self>) -> Rc<dyn SubStateExt> {
        self
    }
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

}

#[cfg(test)]
mod test {
    use super::*;
    struct State;
    impl SubState for State {
        fn state_name(&self) -> &'static str {
            "test_state"
        }
    }
    #[test]
    fn substate() {
        let state = Rc::new(State);
        let state = state.as_state();
        assert_eq!("test_state", state.state_name());
        assert!(matches!(state.as_any().downcast::<State>(), Ok(_))) 
    }
}

