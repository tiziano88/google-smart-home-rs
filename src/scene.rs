use color;
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
        match self.name.as_ref() {
            "Party Mode" => {
                info!("Party Mode");
                for light in &self.lights {
                    let mut l = light.lock().unwrap();
                    l.color_func = Box::new(color::Rainbow { period: 1 });
                }
            }
            "Italian Mode" => {
                info!("Italian Mode");
                for light in &self.lights {
                    let mut l = light.lock().unwrap();
                    l.color_func = Box::new(color::ItalianFlag {});
                }
            }
            "Strobe Mode" => {
                info!("Strobe Mode");
                for light in &self.lights {
                    let mut l = light.lock().unwrap();
                    l.color_func = Box::new(color::Strobe { period: 1 });
                }
            }
            "Night Mode" => {
                info!("Night Mode");
                for light in &self.lights {
                    let mut l = light.lock().unwrap();
                    match l.name.as_ref() {
                        "Bedroom lights" => {
                            l.set_on(false);
                        }
                        "Kitchen lights" => {
                            l.set_on(false);
                        }
                        "Bathroom lights" => {
                            l.set_color(color::RED);
                            l.set_brightness(10);
                        }
                        "Living room lights" => {
                            l.set_on(false);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }
}
