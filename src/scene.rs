use light;
use std::sync::{Arc, Mutex};

pub struct Scene {
    pub id: String,
    pub name: String,
    pub reversible: bool,
    pub lights: Vec<Arc<Mutex<light::Light>>>,
}

impl Scene {
    pub fn activate_scene(&mut self, deactivate: bool) {
        debug!("activate_scene: {:?}", deactivate);
    }
}
