extern crate mote;
extern crate rgb;

use std::str::FromStr;
use std::string::ToString;

use google_actions;

pub struct Scene {
    pub id: String,
    pub name: String,
    pub reversible: bool,
}

impl Scene {
    pub fn activate_scene(&mut self, deactivate: bool) {}
}
