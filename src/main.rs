extern crate iron;
extern crate router;
extern crate params;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate mote;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

use std::collections::BTreeMap;
use std::io::Read;

use iron::headers::ContentType;
use iron::middleware::Handler;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use router::Router;
use url::Url;
use std::sync::Mutex;

mod google_actions;
use google_actions::*;

mod smart_home;
use smart_home::*;

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
                                type_: light.type_.name(),
                                traits: light.available_light_modes
                                    .iter()
                                    .map(LightMode::name)
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
                            })
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                println!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
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
                                    if light.id == request_device.id {}
                                    let light_status = &light.status;
                                    response.payload
                                        .devices
                                        .insert(light.id.clone(),
                                                DeviceStates {
                                                    online: Some(true),
                                                    on: Some(light_status.on),
                                                    brightness: Some(light_status.brightness),
                                                    color: Some(Color {
                                                        name: None,
                                                        temperature: None,
                                                        spectrum_rgb:
                                                            Some(light_status.spectrum_rgb),
                                                    }),
                                                });
                                }
                                &Device::Thermostat(ref thermostat) => {
                                    // TODO
                                }
                            }
                        }
                    }
                }


                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                println!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
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
                            let ref mut devices = *self.devices.lock().unwrap();
                            for device in devices {
                                match device {
                                    &mut Device::Light(ref mut light) => {
                                        println!("execution: {:?}", execution);
                                        if let Some(s) = execution.params.on {
                                            light.set_on(s);
                                        }
                                        if let Some(s) = execution.params.brightness {
                                            light.set_brightness(s);
                                        }
                                        if let Some(ref s) = execution.params.color {
                                            if let Some(s) = s.spectrum_rgb {
                                                light.set_spectrum_rgb(s);
                                            }
                                        }
                                    }
                                    &mut Device::Thermostat(ref thermostat) => {}
                                }
                            }
                        }
                        for request_device in &command.devices {
                            println!("device: {:?}", request_device);
                            let ref devices = *self.devices.lock().unwrap();
                            for device in devices {
                                match device {
                                    &Device::Light(ref light) => {
                                        response.payload.commands.push(ExecuteResponseCommand {
                                            ids: vec![light.id.clone()],
                                            status: "SUCCESS".to_string(),
                                            states: DeviceStates {
                                                online: Some(true),
                                                on: Some(light.status.on),
                                                brightness: Some(light.status.brightness),
                                                color: Some(Color {
                                                    name: None,
                                                    temperature: None,
                                                    spectrum_rgb: Some(light.status.spectrum_rgb),
                                                }),
                                            },
                                        });
                                    }
                                    &Device::Thermostat(ref thermostat) => {
                                        // TODO
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
                return Ok(rsp);
            }
        }

        let mut rsp = Response::with((status::Ok, "ACTION"));
        rsp.headers.set(ContentType::json());
        Ok(rsp)
    }
}

fn main() {
    let hub = Hub {
        devices: Mutex::new(vec![Device::Light(Light {
                                     id: "11".to_string(),
                                     name: "TV lights".to_string(),
                                     status: LightStatus::default(),
                                     type_: LightType::Light,
                                     available_light_modes: vec![LightMode::OnOff,
                                                                 LightMode::Brightness,
                                                                 LightMode::ColorSpectrum],
                                     mote: mote::Mote::new("/dev/ttyACM0"),
                                 })]),
    };
    let mut control = Router::new();
    control.get("/auth", auth_handler, "auth")
        .post("/token", token_handler, "token")
        .get("/login", login_handler, "login")
        .post("/action", hub, "post action")
        .get("/action", get_action_handler, "get action")
        .get("/", index_handler, "index");
    println!("Listening on port 1234");
    Iron::new(control)
        .http("0.0.0.0:1234")
        .unwrap();
}

fn auth_handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<params::Params>().unwrap();

    let _ = map.find(&["response_type"]);
    let _ = map.find(&["client_id"]);
    let redirect_uri = map.find(&["redirect_uri"]);
    let scope = map.find(&["scope"]);
    let state = map.find(&["state"]);

    println!("uri: {:?}", redirect_uri);
    println!("scope: {:?}", scope);

    let s = match state {
        Some(&params::Value::String(ref x)) => x,
        _ => "",
    };

    let u = match redirect_uri {
        Some(&params::Value::String(ref x)) => {
            let mut url = Url::parse(&x).unwrap();
            url.set_query(Some(&format!("code=123&state={}", s)));
            url
        }
        _ => Url::parse("").unwrap(),
    };

    let uu = iron::Url::from_generic_url(u).unwrap();

    Ok(Response::with((status::Found, Redirect(uu))))
}

fn token_handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<params::Params>().unwrap();

    let _ = map.find(&["grant_type"]);
    let _ = map.find(&["code"]);
    let _ = map.find(&["redirect_uri"]);
    let _ = map.find(&["client_id"]);

    let auth_response = AuthResponse {
        token_type: "bearer".to_string(),
        access_token: "xxx".to_string(),
        refresh_token: "yyy".to_string(),
        expires_in: 1000000,
    };

    let res = serde_json::to_string(&auth_response).unwrap_or("".to_string());

    Ok(Response::with((status::Ok, res)))
}

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<params::Params>().unwrap();

    let _ = map.find(&["username"]);
    let _ = map.find(&["password"]);

    Ok(Response::with((status::Ok, "login")))
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}

fn get_action_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "get action")))
}
