use std::ptr::null;

pub struct Asset {
    address: u8,
}

impl Asset {
    pub fn new(address: u8) -> Self {
        Asset { address }
    }

    pub fn get_address(&self) -> u8 {
        self.address
    }
}
