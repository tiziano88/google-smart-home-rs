extern crate mote;
extern crate rgb;

use std::sync::{Arc, Mutex};
use std::string::ToString;

use color;
use google_actions;

pub struct Light {
    pub id: String,
    pub name: String,
    pub status: LightStatus,
    pub available_light_modes: Vec<LightMode>,
    pub type_: LightType,
    pub color_func: Box<color::ColorFunc>,
}

pub enum LightMode {
    OnOff,
    Brightness,
    ColorSpectrum, // TODO: Temperature.
}

impl ToString for LightMode {
    fn to_string(&self) -> String {
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

impl ToString for LightType {
    fn to_string(&self) -> String {
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
    pub fn set_on(&mut self, s: bool) {
        debug!("set_on: {:?}", s);
        self.status.on = s;
        self.output();
    }

    pub fn set_brightness(&mut self, s: u8) {
        debug!("set_brightness: {:?}", s);
        self.status.brightness = s;
        self.status.on = true;
        self.output();
    }

    pub fn set_color(&mut self, c: rgb::RGB8) {
        debug!("set_color: {:?}", c);
        self.status.color = c;
        self.status.on = true;
        self.output();
    }

    fn output(&mut self) {
        let scale = if self.status.on {
            self.status.brightness
        } else {
            0
        } as u32;
        let scaled_r = (self.status.color.r as u32 * scale / 100) as u8;
        let scaled_g = (self.status.color.g as u32 * scale / 100) as u8;
        let scaled_b = (self.status.color.b as u32 * scale / 100) as u8;
        let c = rgb::RGB8 {
            r: scaled_r,
            g: scaled_g,
            b: scaled_b,
        };
        self.color_func = Box::new(color::SolidColor { c: c });
    }
}
