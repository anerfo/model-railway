pub mod model{
    pub use crate::model::Asset;
}

#[derive(Copy, Clone)]
pub enum SwitchState {
    Off,
    On,
    Left,
    Right,
}

pub struct Switch {
    asset: model::Asset,
    state: SwitchState,
}

impl Switch {
    pub fn new(address: u8) -> Self {
        Switch { asset: model::Asset::new(address), state: SwitchState::Off }
    }

    pub fn get_address(&self) -> u8 {
        self.asset.get_address()
    }
    
    pub fn set_state(&mut self, state: SwitchState) {
        self.state = state;
    }

    pub fn get_state(&self) -> SwitchState {
        self.state
    }
}