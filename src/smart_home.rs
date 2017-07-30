extern crate mote;
extern crate rgb;

use google_actions;

pub enum Device {
    Light(Light),
    Thermostat(Thermostat),
}

impl Device {
    pub fn id(&self) -> &str {
        match self {
            &Device::Light(ref light) => &light.id,
            &Device::Thermostat(ref thermostat) => &thermostat.id,
        }
    }
}

pub struct Light {
    pub id: String,
    pub name: String,
    pub status: LightStatus,
    pub available_light_modes: Vec<LightMode>,
    pub type_: LightType,
    pub mote: mote::Mote,
}

pub enum LightMode {
    OnOff,
    Brightness,
    ColorSpectrum, // TODO: Temparature.
}

impl LightMode {
    pub fn name(&self) -> String {
        match self {
            &LightMode::OnOff => "action.devices.traits.OnOff".to_string(),
            &LightMode::Brightness => "action.devices.traits.Brightness".to_string(),
            &LightMode::ColorSpectrum => "action.devices.traits.ColorSpectrum".to_string(),
        }
    }
}

pub enum LightType {
    Light,
    Outlet,
    Switch,
}

impl LightType {
    pub fn name(&self) -> String {
        match self {
            &LightType::Light => "action.devices.types.LIGHT".to_string(),
            &LightType::Outlet => "action.devices.types.OUTLET".to_string(),
            &LightType::Switch => "action.devices.types.SWITCH".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightStatus {
    pub on: bool,
    pub brightness: u8,
    pub color: rgb::RGB8,
}

impl From<google_actions::Params> for LightStatus {
    fn from(params: google_actions::Params) -> LightStatus {
        LightStatus {
            on: params.on.unwrap_or(false),
            brightness: params.brightness.unwrap_or(100),
            color: to_rgb(params.color.unwrap().spectrum_rgb.unwrap_or(0)),
        }
    }
}

impl Into<google_actions::Params> for LightStatus {
    fn into(self) -> google_actions::Params {
        google_actions::Params {
            on: Some(self.on),
            brightness: Some(self.brightness),
            color: Some(google_actions::Color {
                name: None,
                temperature: None,
                spectrum_rgb: Some(from_rgb(&self.color)),
            }),
            ..google_actions::Params::default()
        }
    }
}

impl Default for LightStatus {
    fn default() -> LightStatus {
        LightStatus {
            on: false,
            brightness: 100,
            color: rgb::RGB8 {
                r: 0xFF,
                g: 0xFF,
                b: 0xFF,
            },
        }
    }
}

fn to_rgb(c: u64) -> rgb::RGB8 {
    rgb::RGB8 {
        r: ((c & 0xFF0000) >> 16) as u8,
        g: ((c & 0x00FF00) >> 8) as u8,
        b: ((c & 0x0000FF) >> 0) as u8,
    }
}

fn from_rgb(c: &rgb::RGB8) -> u64 {
    (c.r as u64) << 16 | (c.g as u64) << 8 | (c.b as u64) << 0
}

impl Light {
    pub fn set_on(&mut self, s: bool) -> &LightStatus {
        println!("set on to: {:?}", s);
        self.status.on = s;
        self.output();
        &self.status
    }

    pub fn set_brightness(&mut self, s: u8) -> &LightStatus {
        println!("set brightness to: {:?}", s);
        self.status.brightness = s;
        self.status.on = true;
        self.output();
        &self.status
    }

    pub fn set_color(&mut self, c: rgb::RGB8) -> &LightStatus {
        println!("set color to: {:?}", c);
        self.status.color = c;
        self.status.on = true;
        self.output();
        &self.status
    }

    fn output(&mut self) {
        let scale = if self.status.on {
            self.status.brightness
        } else {
            0
        };
        let scaled_r = (self.status.color.r * scale / 100) as u8;
        let scaled_g = (self.status.color.g * scale / 100) as u8;
        let scaled_b = (self.status.color.b * scale / 100) as u8;
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

impl ThermostatMode {
    pub fn name(&self) -> String {
        match self {
            &ThermostatMode::Off => "Off".to_string(),
            &ThermostatMode::Heat => "Heat".to_string(),
            &ThermostatMode::Cool => "Cool".to_string(),
            &ThermostatMode::On => "On".to_string(),
            &ThermostatMode::Heatcool => "Heatcool".to_string(),
        }
    }
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

impl Into<google_actions::Params> for ThermostatStatus {
    fn into(self) -> google_actions::Params {
        google_actions::Params {
            thermostat_mode: Some(self.mode.name()),
            thermostat_temperature_setpoint: Some(self.temperature_setpoint),
            thermostat_temperature_setpoint_low: Some(self.temperature_setpoint_low),
            thermostat_temperature_setpoint_high: Some(self.temperature_setpoint_high),
            ..google_actions::Params::default()
        }
    }
}

impl Thermostat {
    pub fn temperature_setpoint(&mut self, setpoint: f32) {
        self.status.temperature_setpoint = setpoint;
        self.output();
    }

    pub fn temperature_set_range(&mut self, setpoint_low: f32, setpoint_high: f32) {
        self.status.temperature_setpoint_low = setpoint_low;
        self.status.temperature_setpoint_high = setpoint_high;
        self.output();
    }

    pub fn thermostat_set_mode(&mut self, mode: ThermostatMode) {
        self.status.mode = mode;
        self.output();
    }

    fn output(&mut self) {}
}
