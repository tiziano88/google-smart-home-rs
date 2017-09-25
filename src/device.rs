use light;
use thermostat;
use scene;

use std::sync::{Arc, Mutex, RwLock};

pub enum Device {
    Light(Box<light::Light>),
    Thermostat(Box<thermostat::Thermostat>),
    Scene(Box<scene::Scene>),
}
