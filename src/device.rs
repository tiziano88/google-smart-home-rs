use light;
use thermostat;
use scene;

use google_actions::{ExecuteResponseCommand, Params, SyncResponseDevice};
use std::sync::{Arc, Mutex};

pub enum Device {
    Light(Arc<Mutex<light::Light>>),
    Thermostat(Arc<Mutex<thermostat::Thermostat>>),
    Scene(Arc<Mutex<scene::Scene>>),
    Proxy(String),
}

pub trait DeviceT {
    fn id(&self) -> String;
    fn sync(&self) -> Option<SyncResponseDevice>;
    fn query(&self) -> Option<Params>;
    fn execute(&mut self, &Params) -> Option<ExecuteResponseCommand>;
}
