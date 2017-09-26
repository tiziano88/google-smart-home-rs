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

#[derive(Debug, Clone, PartialEq)]
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
            &ThermostatMode::Off => "off".to_string(),
            &ThermostatMode::Heat => "heat".to_string(),
            &ThermostatMode::Cool => "cool".to_string(),
            &ThermostatMode::On => "on".to_string(),
            &ThermostatMode::Heatcool => "heatcool".to_string(),
        }
    }
}

impl FromStr for ThermostatMode {
    type Err = u8;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(ThermostatMode::Off),
            "heat" => Ok(ThermostatMode::Heat),
            "cool" => Ok(ThermostatMode::Cool),
            "on" => Ok(ThermostatMode::On),
            "heatcool" => Ok(ThermostatMode::Heatcool),
            _ => Err(1),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
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
    pub humidity_ambient: f32,
}

impl Into<google_actions::Params> for ThermostatStatus {
    fn into(self) -> google_actions::Params {
        google_actions::Params {
            thermostat_mode: Some(self.mode.to_string()),
            thermostat_temperature_ambient: Some(self.temperature_ambient),
            thermostat_humidity_ambient: Some(self.humidity_ambient),
            thermostat_temperature_setpoint: Some(self.temperature_setpoint),
            thermostat_temperature_setpoint_low: Some(self.temperature_setpoint_low),
            thermostat_temperature_setpoint_high: Some(self.temperature_setpoint_high),
            ..google_actions::Params::default()
        }
    }
}

impl Thermostat {
    pub fn temperature_setpoint(&mut self, setpoint: f32) {
        debug!("temperature_setpoint: {:?}", setpoint);
        self.status.temperature_setpoint = setpoint;
        if self.status.mode == ThermostatMode::Off {
            self.status.mode = ThermostatMode::On;
        }
        self.output();
    }

    pub fn temperature_set_range(&mut self, setpoint_low: f32, setpoint_high: f32) {
        debug!(
            "temperature_set_range: {:?} - {:?}",
            setpoint_low,
            setpoint_high
        );
        self.status.temperature_setpoint_low = setpoint_low;
        self.status.temperature_setpoint_high = setpoint_high;
        if self.status.mode == ThermostatMode::Off {
            self.status.mode = ThermostatMode::On;
        }
        self.output();
    }

    pub fn thermostat_set_mode(&mut self, mode: ThermostatMode) {
        debug!("thermostat_set_mode: {:?}", mode);
        self.status.mode = mode;
        self.output();
    }

    fn output(&mut self) {
        // TODO
    }
}
