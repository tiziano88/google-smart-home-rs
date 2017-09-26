extern crate env_logger;
extern crate getopts;
extern crate iron;
extern crate mote;
extern crate rgb;
extern crate router;
extern crate scroll_phat_hd;
extern crate serde_json;
extern crate unicase;
extern crate url;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

use std::env;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time;

use getopts::Options;
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
                     SyncResponse, SyncResponseDevice, SyncResponseDeviceAttributes,
                     SyncResponsePayload};

mod light;
use light::{Light, LightMode, LightStatus, LightType};

mod thermostat;
use thermostat::{TemperatureUnit, Thermostat, ThermostatMode, ThermostatStatus};

mod scene;
use scene::Scene;

mod device;
use device::Device;

mod color;

mod oauth;

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

struct Hub {
    devices: Vec<Device>,
}

impl Handler for Hub {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        info!("hub handler");
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();
        let action_request: ActionRequest = serde_json::from_str(&body).unwrap();

        info!("action_request: {:?}", action_request);

        for input in action_request.inputs {
            if input.intent == "action.devices.SYNC" {
                let mut response = SyncResponse {
                    request_id: action_request.request_id.clone(),
                    payload: SyncResponsePayload {
                        agent_user_id: "1111".to_string(),
                        devices: vec![],
                    },
                };

                for device in &self.devices {
                    match device {
                        &Device::Light(ref light) => {
                            let light = light.lock().unwrap();
                            response.payload.devices.push(SyncResponseDevice {
                                id: light.id.clone(),
                                type_: light.type_.to_string(),
                                traits: light
                                    .available_light_modes
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
                            let thermostat = thermostat.lock().unwrap();
                            response.payload.devices.push(SyncResponseDevice {
                                id: thermostat.id.clone(),
                                type_: "action.devices.types.THERMOSTAT".to_string(),
                                traits: vec![
                                    "action.devices.traits.TemperatureSetting".to_string(),
                                ],
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
                                attributes: Some(SyncResponseDeviceAttributes {
                                    available_thermostat_modes: Some(
                                        thermostat
                                            .available_thermostat_modes
                                            .iter()
                                            .map(ToString::to_string)
                                            .collect::<Vec<String>>()
                                            .join(","),
                                    ),
                                    thermostat_temperature_unit: Some(
                                        thermostat.thermostat_temperature_unit.to_string(),
                                    ),
                                    ..SyncResponseDeviceAttributes::default()
                                }),
                            })
                        }
                        &Device::Scene(ref scene) => {
                            let scene = scene.lock().unwrap();
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
                                attributes: Some(SyncResponseDeviceAttributes {
                                    scene_reversible: Some(scene.reversible),
                                    ..SyncResponseDeviceAttributes::default()
                                }),
                            })
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                debug!("action_response: {:?}", res);
                let mut rsp = Response::with((status::Ok, res));
                rsp.headers.set(ContentType::json());
                // For browser access.
                rsp.headers.set(AccessControlAllowOrigin::Any);
                return Ok(rsp);
            } else if input.intent == "action.devices.QUERY" {
                let mut response = QueryResponse {
                    request_id: action_request.request_id.clone(),
                    payload: QueryResponsePayload {
                        devices: btreemap!{},
                    },
                };

                if let Some(payload) = input.payload {
                    for request_device in payload.devices {
                        for device in &self.devices {
                            match device {
                                &Device::Light(ref light) => {
                                    let light = light.lock().unwrap();
                                    if light.id == request_device.id {
                                        response
                                            .payload
                                            .devices
                                            .insert(light.id.clone(), light.status.clone().into());
                                    }
                                }
                                &Device::Thermostat(ref thermostat) => {
                                    let thermostat = thermostat.lock().unwrap();
                                    if thermostat.id == request_device.id {
                                        response.payload.devices.insert(
                                            thermostat.id.clone(),
                                            thermostat.status.clone().into(),
                                        );
                                    }
                                }
                                &Device::Scene(ref _scene) => {}
                            }
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                debug!("action_response: {:?}", res);
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
                        debug!("command: {:?}", command);
                        for execution in &command.execution {
                            debug!("execution: {:?}", execution);
                            for request_device in &command.devices {
                                debug!("request_device: {:?}", request_device);
                                for device in &self.devices {
                                    match device {
                                        &Device::Light(ref light) => {
                                            let mut light = light.lock().unwrap();
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
                                                response.payload.commands.push(
                                                    ExecuteResponseCommand {
                                                        ids: vec![light.id.clone()],
                                                        status: "SUCCESS".to_string(),
                                                        states: light.status.clone().into(),
                                                    },
                                                );
                                            }
                                        }
                                        &Device::Thermostat(ref thermostat) => {
                                            let mut thermostat = thermostat.lock().unwrap();
                                            if thermostat.id == request_device.id {
                                                if let Some(s) =
                                                    execution.params.thermostat_temperature_setpoint
                                                {
                                                    thermostat.temperature_setpoint(s);
                                                }
                                                if let (Some(low), Some(high)) = (
                                                    execution
                                                        .params
                                                        .thermostat_temperature_setpoint_low,
                                                    execution
                                                        .params
                                                        .thermostat_temperature_setpoint_high,
                                                ) {
                                                    thermostat.temperature_set_range(low, high);
                                                }
                                                if let Some(ref mode) =
                                                    execution.params.thermostat_mode
                                                {
                                                    if let Ok(mode) = ThermostatMode::from_str(mode)
                                                    {
                                                        thermostat.thermostat_set_mode(mode);
                                                    }
                                                }
                                                response.payload.commands.push(
                                                    ExecuteResponseCommand {
                                                        ids: vec![thermostat.id.clone()],
                                                        status: "SUCCESS".to_string(),
                                                        states: thermostat.status.clone().into(),
                                                    },
                                                );
                                            }
                                        }
                                        &Device::Scene(ref scene) => {
                                            let mut scene = scene.lock().unwrap();
                                            if scene.id == request_device.id {
                                                scene.activate_scene(
                                                    execution.params.deactivate.unwrap_or(false),
                                                );
                                                response.payload.commands.push(
                                                    ExecuteResponseCommand {
                                                        ids: vec![scene.id.clone()],
                                                        status: "SUCCESS".to_string(),
                                                        states: google_actions::Params::default(),
                                                    },
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let res = serde_json::to_string(&response).unwrap_or("".to_string());
                debug!("action_response: {:?}", res);
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
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("", "http_address", "HTTP address to listen on", "ADDRESS");
    opts.optopt("", "mote_dev", "Serial port connecting to Mote", "FILE");

    debug!("parsing args");
    let matches = opts.parse(&args[1..]).unwrap();
    let http_address = matches
        .opt_str("http_address")
        .unwrap_or("0.0.0.0:1234".to_string());
    let mote_dev = matches
        .opt_str("mote_dev")
        .unwrap_or("/dev/ttyACM0".to_string());
    debug!("args parsed");

    let bedroom_lights = Arc::new(Mutex::new(Light {
        id: "11".to_string(),
        name: "Bedroom lights".to_string(),
        status: LightStatus::default(),
        type_: LightType::Light,
        available_light_modes: vec![
            LightMode::OnOff,
            LightMode::Brightness,
            LightMode::ColorSpectrum,
        ],
        color_func: Box::new(color::SolidColor { c: BLACK }),
    }));

    let kitchen_lights = Arc::new(Mutex::new(Light {
        id: "22".to_string(),
        name: "Kitchen lights".to_string(),
        status: LightStatus::default(),
        type_: LightType::Light,
        available_light_modes: vec![
            LightMode::OnOff,
            LightMode::Brightness,
            LightMode::ColorSpectrum,
        ],
        color_func: Box::new(color::SolidColor { c: BLACK }),
    }));

    let bathroom_lights = Arc::new(Mutex::new(Light {
        id: "33".to_string(),
        name: "Bathroom lights".to_string(),
        status: LightStatus::default(),
        type_: LightType::Light,
        available_light_modes: vec![
            LightMode::OnOff,
            LightMode::Brightness,
            LightMode::ColorSpectrum,
        ],
        color_func: Box::new(color::SolidColor { c: BLACK }),
    }));

    let living_room_lights = Arc::new(Mutex::new(Light {
        id: "44".to_string(),
        name: "Living Room lights".to_string(),
        status: LightStatus::default(),
        type_: LightType::Light,
        available_light_modes: vec![
            LightMode::OnOff,
            LightMode::Brightness,
            LightMode::ColorSpectrum,
        ],
        color_func: Box::new(color::SolidColor { c: BLACK }),
    }));

    let party_mode = Arc::new(Mutex::new(Scene {
        id: "55".to_string(),
        name: "Party mode".to_string(),
        reversible: true,
    }));

    let thermostat = Arc::new(Mutex::new(Thermostat {
        id: "66".to_string(),
        name: "Thermostat".to_string(),
        available_thermostat_modes: vec![ThermostatMode::Off, ThermostatMode::Heat],
        thermostat_temperature_unit: TemperatureUnit::C,
        status: ThermostatStatus {
            mode: ThermostatMode::Off,
            temperature_setpoint: 21.0,
            temperature_ambient: 20.0,
            temperature_setpoint_low: 10.0,
            temperature_setpoint_high: 30.0,
            humidity_ambient: 50.0,
        },
    }));

    let hub = Hub {
        devices: vec![
            Device::Light(bedroom_lights.clone()),
            Device::Light(kitchen_lights.clone()),
            Device::Light(bathroom_lights.clone()),
            Device::Light(living_room_lights.clone()),
            Device::Scene(party_mode.clone()),
            Device::Thermostat(thermostat.clone()),
        ],
    };

    thread::spawn(move || {
        let mut mote = mote::Mote::new(&mote_dev, true);

        let mut pixels = [BLACK; 16 * 4];
        let mut t = 0u64;

        loop {
            for i in 0..16 {
                let offset = 0;
                let lights = bedroom_lights.lock().unwrap();
                let b0 = &pixels.clone()[offset..offset + 16];
                let b1 = lights.color_func.step(t, b0);
                pixels[i + offset] = b1[i];
            }
            for i in 0..16 {
                let offset = 16;
                let lights = kitchen_lights.lock().unwrap();
                let b0 = &pixels.clone()[offset..offset + 16];
                let b1 = lights.color_func.step(t, b0);
                pixels[i + offset] = b1[i];
            }
            for i in 0..16 {
                let offset = 32;
                let lights = bathroom_lights.lock().unwrap();
                let b0 = &pixels.clone()[offset..offset + 16];
                let b1 = lights.color_func.step(t, b0);
                pixels[i + offset] = b1[i];
            }
            for i in 0..16 {
                let offset = 48;
                let lights = living_room_lights.lock().unwrap();
                let b0 = &pixels.clone()[offset..offset + 16];
                let b1 = lights.color_func.step(t, b0);
                pixels[i + offset] = b1[i];
            }
        }
        mote.write(&pixels);

        thread::sleep(time::Duration::from_millis(10));
        t += 1;
    });

    thread::spawn(move || {
        let mut display = scroll_phat_hd::display::TermDisplay::new();
        let mut scroller = scroll_phat_hd::scroller::Scroller::new(&mut display);

        loop {
            {
                let thermostat = thermostat.lock().unwrap();
                info!("ambient: {:?}", thermostat.status.temperature_setpoint);
                scroller.set_text(&format!("{}°C", thermostat.status.temperature_setpoint));
                scroller.show();
            }

            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    let oauth = oauth::OAuth::new();
    let mut control = Router::new();
    control
        .get("/auth", oauth.auth, "auth")
        .post("/token", oauth.token, "token")
        .get("/login", oauth.login, "login")
        .post("/action", hub, "post action")
        .get("/action", get_action_handler, "get action")
        .options("/action", options_action_handler, "get action")
        .get("/", index_handler, "index");
    info!("Listening on {}", http_address);
    Iron::new(control).http(http_address).unwrap();
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
    rsp.headers.set(AccessControlAllowHeaders(
        vec![UniCase("Content-Type".to_string())],
    ));
    Ok(rsp)
}
