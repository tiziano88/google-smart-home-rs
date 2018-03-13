#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate env_logger;
extern crate futures;
extern crate getopts;
extern crate hyper;
extern crate mote;
extern crate reqwest;
extern crate rgb;
extern crate rocket;
extern crate rocket_contrib;
extern crate scroll_phat_hd;
extern crate serde_json;
extern crate staticfile;
extern crate tokio_core;
extern crate unicase;
extern crate url;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use getopts::Options;
use rocket::State;
use rocket::response::Response;
use rocket_contrib::Json;
use unicase::UniCase;
use hyper::header::{AccessControlAllowHeaders, AccessControlAllowOrigin};

mod google_actions;
use google_actions::{ActionRequest, ExecuteResponse, ExecuteResponsePayload, QueryResponse,
                     QueryResponsePayload, SyncResponse, SyncResponsePayload};

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

#[derive(Serialize, Deserialize)]
enum ActionResponse {
    Sync(SyncResponse),
    Query(QueryResponse),
    Execute(ExecuteResponse),
}

struct Hub {
    devices: Vec<Arc<Mutex<Device>>>,
    proxy_urls: Vec<String>,
}

#[post("/action", format = "application/json", data = "<message>")]
fn action(message: Json<ActionRequest>, state: State<Hub>) -> Option<Json<ActionResponse>> {
    info!("action_request: {:?}", message);
    for input in message.0.inputs {
        match input.intent.as_ref() {
            "action.devices.SYNC" => {
                let mut response = SyncResponse {
                    request_id: message.0.request_id.clone(),
                    payload: SyncResponsePayload {
                        agent_user_id: "1111".to_string(),
                        devices: vec![],
                    },
                };

                for device in &state.devices {
                    let device = device.lock().unwrap();
                    response.payload.devices.push(device.sync().unwrap());
                }

                let client = reqwest::Client::new();
                for url in &state.proxy_urls {
                    debug!("proxy url: {:?}", url);
                    // XXX
                    //let mut res = client.post(url).body(body.to_string()).send().unwrap();
                    //debug!("proxy response: {:?}", res);
                    //let proxy_response = res.json::<SyncResponse>().unwrap();
                    //response
                    //.payload
                    //.devices
                    //.extend(proxy_response.payload.devices);
                }

                return Some(Json(ActionResponse::Sync(response)));
            }
            "action.devices.QUERY" => {
                let mut response = QueryResponse {
                    request_id: message.0.request_id.clone(),
                    payload: QueryResponsePayload {
                        devices: btreemap!{},
                    },
                };

                if let Some(payload) = input.payload {
                    for request_device in payload.devices {
                        for device in &state.devices {
                            let device = device.lock().unwrap();
                            if request_device.id == device.id() {
                                response
                                    .payload
                                    .devices
                                    .insert(device.id(), device.query().unwrap());
                            }
                        }
                    }
                }

                let client = reqwest::Client::new();
                for url in &state.proxy_urls {
                    debug!("proxy url: {:?}", url);
                    // XXX
                    //let mut res = client.post(url).body(body.to_string()).send().unwrap();
                    //debug!("proxy response: {:?}", res);
                    //let proxy_response = res.json::<QueryResponse>().unwrap();
                    //response
                    //.payload
                    //.devices
                    //.extend(proxy_response.payload.devices);
                }

                return Some(Json(ActionResponse::Query(response)));
            }
            "action.devices.EXECUTE" => {
                let mut response = ExecuteResponse {
                    request_id: message.0.request_id.clone(),
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
                                for device in &state.devices {
                                    let mut device = device.lock().unwrap();
                                    if request_device.id == device.id() {
                                        response
                                            .payload
                                            .commands
                                            .push(device.execute(&execution.params).unwrap());
                                    }
                                }
                            }
                        }
                    }
                }

                let client = reqwest::Client::new();
                for url in &state.proxy_urls {
                    debug!("proxy url: {:?}", url);
                    // XXX
                    //let mut res = client.post(url).body(body.to_string()).send().unwrap();
                    //debug!("proxy response: {:?}", res);
                    //let proxy_response = res.json::<ExecuteResponse>().unwrap();
                    //response
                    //.payload
                    //.commands
                    //.extend(proxy_response.payload.commands);
                }

                return Some(Json(ActionResponse::Execute(response)));
            }
            i => {
                debug!("unsupported intent: {:?}", i);
                return None;
            }
        }
    }

    return None;
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("", "http_address", "HTTP address to listen on", "ADDRESS");
    opts.optopt("", "index", "file to serve as index", "FILE");
    opts.optopt("", "mote_dev", "Serial port connecting to Mote", "FILE");
    opts.optopt("", "display_i2c", "I2C port to use as display", "N");

    debug!("parsing args");
    let matches = opts.parse(&args[1..]).unwrap();
    let http_address = matches
        .opt_str("http_address")
        .unwrap_or("0.0.0.0:1234".to_string());
    let index = matches.opt_str("index").unwrap_or("/dev/null".to_string());
    let mote_dev = matches
        .opt_str("mote_dev")
        .unwrap_or("/dev/ttyACM0".to_string());
    let display_i2c = matches.opt_str("display_i2c").unwrap_or("".to_string());
    debug!("args parsed");

    let bedroom_lights = Arc::new(Mutex::new(Light {
        id: "111".to_string(),
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
        id: "222".to_string(),
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
        id: "333".to_string(),
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
        id: "444".to_string(),
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

    let all_lights = vec![
        bedroom_lights.clone(),
        kitchen_lights.clone(),
        bathroom_lights.clone(),
        living_room_lights.clone(),
    ];

    let party_mode = Arc::new(Mutex::new(Scene {
        id: "1001".to_string(),
        name: "Party mode".to_string(),
        reversible: true,
        lights: all_lights.clone(),
    }));

    let italian_mode = Arc::new(Mutex::new(Scene {
        id: "1002".to_string(),
        name: "Italian Mode".to_string(),
        reversible: true,
        lights: all_lights.clone(),
    }));

    let night_mode = Arc::new(Mutex::new(Scene {
        id: "1003".to_string(),
        name: "Night Mode".to_string(),
        reversible: true,
        lights: all_lights.clone(),
    }));

    let strobe_mode = Arc::new(Mutex::new(Scene {
        id: "1004".to_string(),
        name: "Strobe Mode".to_string(),
        reversible: true,
        lights: all_lights.clone(),
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
            bedroom_lights.clone(),
            kitchen_lights.clone(),
            bathroom_lights.clone(),
            living_room_lights.clone(),
            party_mode.clone(),
            italian_mode.clone(),
            night_mode.clone(),
            strobe_mode.clone(),
            thermostat.clone(),
        ],
        proxy_urls: vec![],
    };

    thread::spawn(move || {
        let mut mote = mote::Mote::new(&mote_dev, true);

        let mut pixels = [BLACK; 16 * 4];
        let mut t = 0u64;

        fn update_lights(
            pixels: &mut [rgb::RGB8; 16 * 4],
            t: u64,
            lights: &Arc<Mutex<Light>>,
            offset: usize,
        ) {
            match lights.lock() {
                Ok(lights) => for i in 0..16 {
                    let b0 = &pixels.clone()[offset..offset + 16];
                    let b1 = lights.color_func.step(t, b0);
                    pixels[i + offset] = b1[i];
                },
                Err(err) => error!("could not lock light mutex: {:?}", err),
            }
        }

        loop {
            update_lights(&mut pixels, t, &bedroom_lights, 0);
            update_lights(&mut pixels, t, &kitchen_lights, 16);
            update_lights(&mut pixels, t, &bathroom_lights, 32);
            update_lights(&mut pixels, t, &living_room_lights, 48);
            mote.write(&pixels);

            thread::sleep(time::Duration::from_millis(10));
            t += 1;
        }
    });

    thread::spawn(move || {
        // XXX
        return;
        let mut display: Box<scroll_phat_hd::display::Display> = if display_i2c == "" {
            Box::new(scroll_phat_hd::display::UnicodeDisplay::new())
        } else {
            // TODO: Parse I2C port.
            Box::new(scroll_phat_hd::display::I2CDisplay::new(1))
        };

        let mut scroller = scroll_phat_hd::scroller::Scroller::new(&mut *display);

        loop {
            match thermostat.lock() {
                Ok(thermostat) => {
                    match thermostat.status.mode {
                        thermostat::ThermostatMode::Off => {
                            scroller.set_text("--°C");
                            scroller.show();
                        }
                        _ => {
                            scroller.set_text(&format!(
                                "{}°C",
                                thermostat.status.temperature_setpoint
                            ));
                            scroller.show();
                        }
                    };
                }
                Err(err) => {
                    error!("could not lock thermostat mutex: {:?}", err);
                }
            };

            thread::sleep(time::Duration::from_millis(100));
        }
    });

    rocket::ignite()
        .manage(hub)
        .mount(
            "/",
            routes![
                get_action_handler,
                options_action_handler,
                action,
                oauth::auth,
                oauth::token,
                oauth::login,
            ],
        )
        .launch();
}

#[get("/action")]
fn get_action_handler() -> String {
    "get action".to_string()
}

#[options("/action")]
fn options_action_handler() -> Response<'static> {
    Response::build()
        // XXX
        //.header(AccessControlAllowOrigin::Any)
        //.header(AccessControlAllowHeaders(vec![
        //UniCase("Content-Type".to_string()),
        //]))
        .finalize()
}
