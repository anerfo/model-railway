use crate::interface::IClient;

pub mod model{
    pub use crate::model::Asset;
}

pub struct Engine {
    asset: model::Asset,
    speed: u8,
    function: [bool; 16],
}

impl Engine {
    pub fn new(address: u8, site: &'a dyn IClient>) -> Self {
        Engine { asset: model::Asset::new(address, site), speed: 0, function: [false; 16] }
    }

    pub fn get_address(&self) -> u8 {
        self.asset.get_address()
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed.min(14);
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn set_function(&mut self, index: usize, state: bool) {
        if index < self.function.len() {
            self.function[index] = state;
        }
    }

    pub fn get_function(&self, index: usize) -> Option<bool> {
        if index < self.function.len() {
            Some(self.function[index])
        } else {
            None
        }
    }
}
