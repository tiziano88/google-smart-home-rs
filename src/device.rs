extern crate mote;
extern crate rgb;

use std::str::FromStr;
use std::string::ToString;

use light;
use thermostat;
use scene;

use google_actions;

pub enum Device {
    Light(light::Light),
    Thermostat(thermostat::Thermostat),
    Scene(light::Light),
}

impl Device {
    pub fn id(&self) -> &str {
        match self {
            &Device::Light(ref light) => &light.id,
            &Device::Thermostat(ref thermostat) => &thermostat.id,
            &Device::Scene(ref scene) => &scene.id,
        }
    }
}
