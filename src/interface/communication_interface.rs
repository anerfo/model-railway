use crate::model::{Engine, Switch};

pub trait ICommunication: Send + Sync {
    fn engine_state_changed(&mut self, engine: &Engine);
    fn switch_state_changed(&mut self, switch: &Switch);
}
