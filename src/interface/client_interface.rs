use crate::model::{Engine, Switch, Status};

pub trait IClient: Send + Sync {
    fn engine_state_changed(&mut self, engine: &Engine);
    fn switch_state_changed(&mut self, switch: &Switch);
    fn status_state_changed(&mut self, status: &Status);
}
