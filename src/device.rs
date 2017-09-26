use light;
use thermostat;
use scene;

use std::sync::{Arc, Mutex};

pub enum Device {
    Light(Arc<Mutex<light::Light>>),
    Thermostat(Arc<Mutex<thermostat::Thermostat>>),
    Scene(Arc<Mutex<scene::Scene>>),
}
