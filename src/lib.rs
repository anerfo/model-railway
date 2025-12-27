pub mod model {
    pub mod asset;
    pub use asset::Asset;
    pub mod engine;
    pub use engine::Engine;
    pub mod switch;
    pub use switch::Switch;
    pub mod status;
    pub use status::Status;
    pub mod site;
    pub use site::Site;
}

pub mod interface {
    pub mod communication_interface;
    pub use communication_interface::ICommunication;
    pub mod client_interface;
    pub use client_interface::IClient;
}
