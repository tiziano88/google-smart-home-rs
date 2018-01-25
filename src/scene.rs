use color;
use light;

use device::DeviceT;
use google_actions::{ExecuteResponseCommand, Name, Params, SyncResponseDevice,
                     SyncResponseDeviceAttributes};
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

impl DeviceT for Scene {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn sync(&self) -> Option<SyncResponseDevice> {
        Option::Some(SyncResponseDevice {
            id: self.id(),
            type_: "action.devices.types.SCENE".to_string(),
            traits: vec!["action.devices.traits.Scene".to_string()],
            name: Name {
                default_name: vec![self.name.to_string()],
                name: Some(self.name.clone()),
                nicknames: vec![],
            },
            will_report_state: false,
            device_info: None,
            room_hint: None,
            structure_hint: None,
            attributes: Some(SyncResponseDeviceAttributes {
                scene_reversible: Some(self.reversible),
                ..SyncResponseDeviceAttributes::default()
            }),
        })
    }

    fn query(&self) -> Option<Params> {
        Option::None
    }

    fn execute(&mut self, params: &Params) -> Option<ExecuteResponseCommand> {
        self.activate_scene(params.deactivate.unwrap_or(false));
        Option::Some(ExecuteResponseCommand {
            ids: vec![self.id()],
            status: "SUCCESS".to_string(),
            states: Params::default(),
        })
    }
}
