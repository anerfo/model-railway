pub mod model{
    pub use crate::model::Engine;
    pub use crate::model::Switch;
    pub use crate::model::Status;
}

pub struct Site<const ENGINE_COUNT: usize, const SWITCH_COUNT: usize, const STATUS_COUNT: usize> {
    engine: [model::Engine; ENGINE_COUNT],
    switch: [model::Switch; SWITCH_COUNT],
    status: [model::Status; STATUS_COUNT],
    communication: Box<dyn crate::interface::ICommunication>,
    clients: Vec<Box<dyn crate::interface::IClient>>,
}

impl<const ENGINE_COUNT: usize, const SWITCH_COUNT: usize, const STATUS_COUNT: usize> Site<ENGINE_COUNT, SWITCH_COUNT, STATUS_COUNT> {
    pub fn new(communication: Box<dyn crate::interface::ICommunication>) -> Self {
        Site {
            engine: core::array::from_fn(|i| model::Engine::new(i as u8, &self)),
            switch: core::array::from_fn(|i| model::Switch::new(i as u8, &self)),
            status: core::array::from_fn(|i| model::Status::new(i as u8)),
            communication,
            clients: Vec::new(),
        }
    }

    pub fn get_engine(&mut self, index: usize) -> &mut model::Engine {
        &mut self.engine[index]
    }
    pub fn get_switch(&mut self, index: usize) -> &mut model::Switch {
        &mut self.switch[index]
    }
    pub fn get_status(&mut self, index: usize) -> &mut model::Status {
        &mut self.status[index]
    }

    pub fn subscribe<T: crate::interface::IClient>(&mut self, _interface: T) 
    where
        T: 'static,
    {
        self.clients.push(Box::new(_interface));
    }
}

impl<const ENGINE_COUNT: usize, const SWITCH_COUNT: usize, const STATUS_COUNT: usize> crate::interface::IClient for Site<ENGINE_COUNT, SWITCH_COUNT, STATUS_COUNT> {
    fn engine_state_changed(&mut self, engine: &model::Engine) {
        self.communication.engine_state_changed(engine);
        for client in &mut self.clients {
            client.engine_state_changed(engine);
        }
    }

    fn switch_state_changed(&mut self, switch: &model::Switch) {
        self.communication.switch_state_changed(switch);
        for client in &mut self.clients {
            client.switch_state_changed(switch);
        }
    }

    fn status_state_changed(&mut self, status: &model::Status) {
        for client in &mut self.clients {
            client.status_state_changed(status);
        }
    }
}