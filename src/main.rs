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
    lights: Mutex<Vec<Light>>,
}

impl Handler for Hub {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        println!("hub handler");
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();
        let action_request: ActionRequest = serde_json::from_str(&body).unwrap();

        println!("action_request: {:?}", action_request);

        for input in &action_request.inputs {
            if input.intent == "action.devices.SYNC" {
                let mut response = SyncResponse {
                    request_id: action_request.request_id.clone(),
                    payload: SyncResponsePayload {
                        agent_user_id: "1111".to_string(),
                        devices: vec![],
                    },
                };

                let ref lights = *self.lights.lock().unwrap();
                for light in lights {
                    response.payload.devices.push(SyncResponseDevice {
                        id: light.id.clone(),
                        type_: "action.devices.types.LIGHT".to_string(),
                        traits: vec!["action.devices.traits.OnOff".to_string(),
                                     "action.devices.traits.Brightness".to_string(),
                                     "action.devices.traits.ColorSpectrum".to_string()],
                        name: Name {
                            default_name: vec![light.name.to_string()],
                            name: Some(light.name.clone()),
                            nicknames: vec![],
                        },
                        will_report_state: false,
                        device_info: None,
                        room_hint: None,
                        structure_hint: None,
                    });
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

                let ref lights = *self.lights.lock().unwrap();
                for light in lights {
                    let light_status = &light.status;
                    response.payload.devices.insert(light.id.clone(),
                                                    DeviceStates {
                                                        online: true,
                                                        on: light_status.on,
                                                        brightness: light_status.brightness,
                                                        color: Color {
                                                            name: None,
                                                            temperature: None,
                                                            spectrum_rgb:
                                                                Some(light_status.spectrum_rgb),
                                                        },
                                                    });
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
                            let ref mut lights = *self.lights.lock().unwrap();
                            for light in lights {
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
                        }
                        for device in &command.devices {
                            println!("device: {:?}", device);
                            let ref lights = *self.lights.lock().unwrap();
                            for light in lights {
                                response.payload.commands.push(ExecuteResponseCommand {
                                    ids: vec![light.id.clone()],
                                    status: "SUCCESS".to_string(),
                                    states: DeviceStates {
                                        online: true,
                                        on: light.status.on,
                                        brightness: light.status.brightness,
                                        color: Color {
                                            name: None,
                                            temperature: None,
                                            spectrum_rgb: Some(light.status.spectrum_rgb),
                                        },
                                    },
                                });
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
    let mut hub = Hub {
        lights: Mutex::new(vec![Light {
                                    id: "11".to_string(),
                                    name: "TV lights".to_string(),
                                    status: LightStatus::default(),
                                    mote: mote::Mote::new("/dev/ttyACM0"),
                                }]),
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

    let redirect_uri = map.find(&["response_type"]);
    let client_id = map.find(&["client_id"]);
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

    let grant_type = map.find(&["grant_type"]);
    let code = map.find(&["code"]);
    let redirect_uri = map.find(&["redirect_uri"]);
    let client_id = map.find(&["client_id"]);

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

    let username = map.find(&["username"]);
    let password = map.find(&["password"]);

    Ok(Response::with((status::Ok, "login")))
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}

fn get_action_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "get action")))
}
