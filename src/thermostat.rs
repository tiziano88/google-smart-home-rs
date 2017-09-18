use std::str::FromStr;
use std::string::ToString;

use google_actions;

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

impl ToString for ThermostatMode {
    fn to_string(&self) -> String {
        match self {
            &ThermostatMode::Off => "Off".to_string(),
            &ThermostatMode::Heat => "Heat".to_string(),
            &ThermostatMode::Cool => "Cool".to_string(),
            &ThermostatMode::On => "On".to_string(),
            &ThermostatMode::Heatcool => "Heatcool".to_string(),
        }
    }
}

impl FromStr for ThermostatMode {
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Off" => Ok(ThermostatMode::Off),
            "Heat" => Ok(ThermostatMode::Heat),
            "Cool" => Ok(ThermostatMode::Cool),
            "On" => Ok(ThermostatMode::On),
            "Heatcool" => Ok(ThermostatMode::Heatcool),
            _ => Err(1),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TemperatureUnit {
    C,
    F,
}

impl ToString for TemperatureUnit {
    fn to_string(&self) -> String {
        match self {
            &TemperatureUnit::C => "C".to_string(),
            &TemperatureUnit::F => "F".to_string(),
        }
    }
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
            thermostat_mode: Some(self.mode.to_string()),
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

    fn output(&mut self) {
        // TODO
    }
}
