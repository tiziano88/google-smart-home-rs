use light;
use thermostat;
use scene;

pub enum Device {
    Light(light::Light),
    Thermostat(thermostat::Thermostat),
    Scene(scene::Scene),
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
