use crate::interface::IClient;

pub struct Asset {
    address: u8,
    client: &dyn IClient,
}

impl Asset {
    pub fn new(address: u8, client: &dyn IClient) -> Self {
        Asset { address, client }
    }

    pub fn get_address(&self) -> u8 {
        self.address
    }
}
