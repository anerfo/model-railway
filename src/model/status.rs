pub mod model{
    pub use crate::model::Asset;
}

#[derive(Copy, Clone)]
pub enum StatusState {
    Off,
    On,
}

pub struct Status {
    asset: model::Asset,
    state: StatusState,
}

impl Status {
    pub fn new(address: u8) -> Self {
        Status { asset: model::Asset::new(address), state: StatusState::Off }
    }

    pub fn get_address(&self) -> u8 {
        self.asset.get_address()
    }

    pub fn set_state(&mut self, state: StatusState) {
        self.state = state;
    }

    pub fn get_state(&self) -> StatusState {
        self.state
    }
}