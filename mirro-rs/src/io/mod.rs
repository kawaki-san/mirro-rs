use std::time::Duration;

pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialise,      // Launch to init application
    Sleep(Duration), // Take a little break
}
