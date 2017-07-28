extern crate mote;
extern crate rgb;

use std::process::Command;

pub enum Device {
    Light(Light),
    Thermostat(Thermostat),
}

pub struct Light {
    pub id: String,
    pub name: String,
    pub status: LightStatus,
    pub type_: LightType,
    pub mote: mote::Mote,
}

pub enum LightType {
    Light,
    Outlet,
    Switch,
}

impl LightType {
    pub fn name(&self) -> String {
        match self {
            Light => "action.devices.types.LIGHT".to_string(),
            Outlet => "action.devices.types.OUTLET".to_string(),
            Switch => "action.devices.types.SWITCH".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightStatus {
    pub on: bool,
    pub brightness: u64,
    pub spectrum_rgb: u64,
}

impl Default for LightStatus {
    fn default() -> LightStatus {
        LightStatus {
            on: false,
            brightness: 100,
            spectrum_rgb: 0xFFFFFF,
        }
    }
}

impl Light {
    pub fn set_on(&mut self, s: bool) -> &LightStatus {
        println!("set on to: {:?}", s);
        self.status.on = s;
        self.output();
        &self.status
    }

    pub fn set_brightness(&mut self, s: u64) -> &LightStatus {
        println!("set brightness to: {:?}", s);
        self.status.brightness = s;
        self.status.on = true;
        self.output();
        &self.status
    }

    pub fn set_spectrum_rgb(&mut self, s: u64) -> &LightStatus {
        println!("set spectrum_rgb to: {:?}", s);
        self.status.spectrum_rgb = s;
        self.status.on = true;
        self.output();
        &self.status
    }

    fn output(&mut self) {
        let r = (self.status.spectrum_rgb & 0xFF0000) >> 16;
        let g = (self.status.spectrum_rgb & 0x00FF00) >> 8;
        let b = (self.status.spectrum_rgb & 0x0000FF) >> 0;
        let scale = if self.status.on {
            self.status.brightness
        } else {
            0
        };
        let scaled_r = (r * scale / 100) as u8;
        let scaled_g = (g * scale / 100) as u8;
        let scaled_b = (b * scale / 100) as u8;
        let c = rgb::RGB8 {
            r: scaled_r,
            g: scaled_g,
            b: scaled_b,
        };
        self.mote.write(&[c; 16 * 4]);
    }
}

pub struct Thermostat {
    pub id: String,
    pub name: String,
    pub available_thermostat_modes: Vec<ThermostatMode>,
    pub thermostat_temperature_unit: TemperatureUnit,
    pub status: ThermostatStatus,
}

#[derive(Debug, Clone)]
pub enum ThermostatMode {
    Off,
    Heat,
    Cool,
    On,
    Heatcool,
}

#[derive(Debug, Clone)]
pub enum TemperatureUnit {
    C,
    F,
}

#[derive(Debug, Clone)]
pub struct ThermostatStatus {
    pub mode: ThermostatMode,
    pub temperature_setpoint: f32,
    pub temperature_ambient: f32,
    pub temperature_setpoint_low: f32,
    pub temperature_setpoint_high: f32,
    pub temperature_humidity_ambient: f32,
}

impl Thermostat {
    pub fn temperature_setpoint(&mut self, setpoint: f32) {
        self.status.temperature_setpoint = setpoint;
    }

    pub fn temperature_set_range(&mut self, setpoint_low: f32, setpoint_high: f32) {
        self.status.temperature_setpoint_low = setpoint_low;
        self.status.temperature_setpoint_high = setpoint_high;
    }

    pub fn thermostat_set_mode(&mut self, mode: ThermostatMode) {
        self.status.mode = mode;
    }

    fn output(&mut self) {}
}
