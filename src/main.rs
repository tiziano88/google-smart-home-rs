extern crate iron;
extern crate mote;
extern crate rgb;
extern crate router;
extern crate serde_json;
extern crate unicase;
extern crate url;
extern crate coap;

use std::io;
use coap::{CoAPServer, CoAPClient, CoAPRequest, CoAPResponse};

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use iron::headers::{AccessControlAllowHeaders, AccessControlAllowOrigin, ContentType};
use iron::middleware::Handler;
use iron::prelude::{IronResult, Request, Response};
use iron::prelude::*;
use iron::status;
use router::Router;
use unicase::UniCase;

mod google_actions;
use google_actions::{ActionRequest, ExecuteResponse, ExecuteResponseCommand,
                     ExecuteResponsePayload, Name, QueryResponse, QueryResponsePayload,
                     SyncResponse, SyncResponseDevice, SyncResponsePayload};

mod light;
use light::{Light, LightMode, LightStatus, LightType};

mod thermostat;
use thermostat::ThermostatMode;

mod scene;
use scene::Scene;

mod device;
use device::Device;

mod oauth;

struct Hub {
    devices: Mutex<Vec<Device>>,
}

impl Handler for Hub {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("hub handler");
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();
        let action_request: ActionRequest = serde_json::from_str(&body).unwrap();

        println!("action_request: {:?}", action_request);

        for input in action_request.inputs {
            if input.intent == "action.devices.SYNC" {
                let mut response = SyncResponse {
                    request_id: action_request.request_id.clone(),
                    payload: SyncResponsePayload {
                        agent_user_id: "1111".to_string(),
                        devices: vec![],
                    },
                };

                let ref devices = *self.devices.lock().unwrap();
                for device in devices {
                    match device {
                        &Device::Light(ref light) => {
                            response.payload.devices.push(SyncResponseDevice {
                                id: light.id.clone(),
                                type_: light.type_.to_string(),
                                traits: light.available_light_modes
                                    .iter()
                                    .map(LightMode::to_string)
                                    .collect(),
                                name: Name {
                                    default_name: vec![light.name.to_string()],
                                    name: Some(light.name.clone()),
                                    nicknames: vec![],
                                },
                                will_report_state: false,
                                device_info: None,
                                room_hint: None,
                                structure_hint: None,
                                attributes: None,
                            })
                        }
                        &Device::Thermostat(ref thermostat) => {
                            response.payload.devices.push(SyncResponseDevice {
                                id: thermostat.id.clone(),
                                type_: "".to_string(), // XXX
                                traits: vec!["action.devices.traits.TemperatureSetting"
                                                 .to_string()],
                                name: Name {
                                    default_name: vec![thermostat.name.to_string()],
                                    name: Some(thermostat.name.clone()),
                                    nicknames: vec![],
                                },
                                // TODO: attributes.
                                will_report_state: false,
                                device_info: None,
                                room_hint: None,
                                structure_hint: None,
                                attributes: None,
                            })
                        }
                        &Device::Scene(ref scene) => {
                            response.payload.devices.push(SyncResponseDevice {
                                id: scene.id.clone(),
                                type_: "action.devices.types.SCENE".to_string(),
                                traits: vec!["action.devices.traits.Scene".to_string()],
                                name: Name {
                                    default_name: vec![scene.name.to_string()],
                                    name: Some(scene.name.clone()),
                                    nicknames: vec![],
                                },
                                will_report_state: false,
                                device_info: None,
                                room_hint: None,
                                structure_hint: None,
                                attributes: None,
                            })
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                println!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
                // For browser access.
                rsp.headers.set(AccessControlAllowOrigin::Any);
                return Ok(rsp);
            } else if input.intent == "action.devices.QUERY" {
                let mut response = QueryResponse {
                    request_id: action_request.request_id.clone(),
                    payload: QueryResponsePayload { devices: btreemap!{} },
                };

                let ref devices = *self.devices.lock().unwrap();
                if let Some(payload) = input.payload {
                    for request_device in payload.devices {
                        for device in devices {
                            match device {
                                &Device::Light(ref light) => {
                                    if light.id == request_device.id {
                                        response.payload
                                            .devices
                                            .insert(light.id.clone(), light.status.clone().into());
                                    }
                                }
                                &Device::Thermostat(ref thermostat) => {
                                    if thermostat.id == request_device.id {
                                        // TODO
                                    }
                                }
                                &Device::Scene(ref _scene) => {}
                            }
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                println!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
                // For browser access.
                rsp.headers.set(AccessControlAllowOrigin::Any);
                return Ok(rsp);
            } else if input.intent == "action.devices.EXECUTE" {
                let mut response = ExecuteResponse {
                    request_id: action_request.request_id.clone(),
                    payload: ExecuteResponsePayload {
                        error_code: None,
                        debug_string: None,
                        commands: vec![],
                    },
                };

                if let Some(ref p) = input.payload {
                    for command in &p.commands {
                        println!("command: {:?}", command);
                        for execution in &command.execution {
                            println!("execution: {:?}", execution);
                            for request_device in &command.devices {
                                println!("request_device: {:?}", request_device);
                                let ref mut devices = *self.devices.lock().unwrap();
                                for device in devices {
                                    match device {
                                        &mut Device::Light(ref mut light) => {
                                            if light.id == request_device.id {
                                                if let Some(s) = execution.params.on {
                                                    light.set_on(s);
                                                }
                                                if let Some(s) = execution.params.brightness {
                                                    light.set_brightness(s);
                                                }
                                                if let Some(ref s) = execution.params.color {
                                                    if let Some(s) = s.spectrum_rgb {
                                                        light.set_color(to_rgb(s));
                                                    }
                                                }
                                                response.payload
                                                    .commands
                                                    .push(ExecuteResponseCommand {
                                                        ids: vec![light.id.clone()],
                                                        status: "SUCCESS".to_string(),
                                                        states: light.status.clone().into(),
                                                    });
                                            }
                                        }
                                        &mut Device::Thermostat(ref mut thermostat) => {
                                            if thermostat.id == request_device.id {
                                                if let Some(s) = execution.params
                                                    .thermostat_temperature_setpoint {
                                                    thermostat.temperature_setpoint(s);
                                                }
                                                if let (Some(low), Some(high)) =
                                                    (execution.params
                                                         .thermostat_temperature_setpoint_low,
                                                     execution.params
                                                         .thermostat_temperature_setpoint_high) {
                                                    thermostat.temperature_set_range(low, high);
                                                }
                                                if let Some(ref mode) = execution.params
                                                    .thermostat_mode {
                                                    if let Ok(mode) =
                                                        ThermostatMode::from_str(mode) {
                                                        thermostat.thermostat_set_mode(mode);
                                                    }
                                                }
                                                response.payload
                                                    .commands
                                                    .push(ExecuteResponseCommand {
                                                        ids: vec![thermostat.id.clone()],
                                                        status: "SUCCESS".to_string(),
                                                        states: thermostat.status.clone().into(),
                                                    });
                                            }
                                        }
                                        &mut Device::Scene(ref mut _scene) => {}
                                    }
                                }
                            }
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                println!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
                // For browser access.
                rsp.headers.set(AccessControlAllowOrigin::Any);
                return Ok(rsp);
            }
        }

        let mut rsp = Response::with((status::Ok, "ACTION"));
        rsp.headers.set(ContentType::json());
        // For browser access.
        rsp.headers.set(AccessControlAllowOrigin::Any);
        Ok(rsp)
    }
}

fn to_rgb(c: u64) -> rgb::RGB8 {
    rgb::RGB8 {
        r: ((c & 0xFF0000) >> 16) as u8,
        g: ((c & 0x00FF00) >> 8) as u8,
        b: ((c & 0x0000FF) >> 0) as u8,
    }
}

fn main() {
    let mote = Arc::new(Mutex::new(mote::Mote::new("/dev/ttyACM0", true)));
    let hub = Hub {
        devices: Mutex::new(vec![Device::Light(Light {
                                     id: "11".to_string(),
                                     name: "Bedroom lights".to_string(),
                                     status: LightStatus::default(),
                                     type_: LightType::Light,
                                     available_light_modes: vec![LightMode::OnOff,
                                                                 LightMode::Brightness,
                                                                 LightMode::ColorSpectrum],
                                     mote: mote.clone(),
                                     pixel_low: 0,
                                     pixel_high: 16,
                                 }),
                                 Device::Light(Light {
                                     id: "22".to_string(),
                                     name: "Kitchen lights".to_string(),
                                     status: LightStatus::default(),
                                     type_: LightType::Light,
                                     available_light_modes: vec![LightMode::OnOff,
                                                                 LightMode::Brightness,
                                                                 LightMode::ColorSpectrum],
                                     mote: mote.clone(),
                                     pixel_low: 16,
                                     pixel_high: 32,
                                 }),
                                 Device::Light(Light {
                                     id: "33".to_string(),
                                     name: "Bathroom lights".to_string(),
                                     status: LightStatus::default(),
                                     type_: LightType::Light,
                                     available_light_modes: vec![LightMode::OnOff,
                                                                 LightMode::Brightness,
                                                                 LightMode::ColorSpectrum],
                                     mote: mote.clone(),
                                     pixel_low: 32,
                                     pixel_high: 48,
                                 }),
                                 Device::Light(Light {
                                     id: "44".to_string(),
                                     name: "Living Room lights".to_string(),
                                     status: LightStatus::default(),
                                     type_: LightType::Light,
                                     available_light_modes: vec![LightMode::OnOff,
                                                                 LightMode::Brightness,
                                                                 LightMode::ColorSpectrum],
                                     mote: mote.clone(),
                                     pixel_low: 48,
                                     pixel_high: 64,
                                 }),
                                 Device::Scene(Scene {
                                     id: "55".to_string(),
                                     name: "Party mode".to_string(),
                                     reversible: false,
                                 })]),
    };
    let oauth = oauth::OAuth::new();
    let mut control = Router::new();
    control.get("/auth", oauth.auth, "auth")
        .post("/token", oauth.token, "token")
        .get("/login", oauth.login, "login")
        .post("/action", hub, "post action")
        .get("/action", get_action_handler, "get action")
        .options("/action", options_action_handler, "get action")
        .get("/", index_handler, "index");

    let addr = "0.0.0.0:5684";

    let mut server = CoAPServer::new(addr).unwrap();
    server.handle(request_handler).unwrap();

    println!("Listening on port 1234");
    Iron::new(control).http("0.0.0.0:1234").unwrap();
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}

fn get_action_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "get action")))
}

fn options_action_handler(_: &mut Request) -> IronResult<Response> {
    let mut rsp = Response::with((status::Ok, "options"));
    rsp.headers.set(AccessControlAllowOrigin::Any);
    rsp.headers.set(AccessControlAllowHeaders(vec![UniCase("Content-Type".to_string())]));
    Ok(rsp)
}

fn request_handler(request: CoAPRequest) -> Option<CoAPResponse> {
    println!("Receive request: {:?}", request);

    // Return the auto-generated response
    request.response
}
