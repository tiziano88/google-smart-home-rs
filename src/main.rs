extern crate iron;
extern crate router;
extern crate params;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate maplit;

use std::collections::BTreeMap;
use std::io::Read;

use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use router::Router;
use url::Url;

mod google_actions;
use google_actions::*;

mod smart_home;
use smart_home::*;

fn main() {
    let mut light = Light {
        id: "Foo".to_string(),
        name: "Foo".to_string(),
    };
    println!("Hello, world!");
    let mut control = Router::new();
    control.get("/auth", auth_handler, "auth")
        .post("/token", token_handler, "token")
        .get("/login", login_handler, "login")
        .post("/action", action_handler, "action")
        .get("/", index_handler, "index");
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

fn action_handler(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    req.body.read_to_string(&mut body).unwrap();
    let action_request: ActionRequest = serde_json::from_str(&body).unwrap();

    println!("action_request: {:?}", action_request);

    for input in &action_request.inputs {
        if input.intent == "action.devices.SYNC" {
            let response = SyncResponse {
                request_id: action_request.request_id.clone(),
                payload: SyncResponsePayload {
                    agent_user_id: "1111".to_string(),
                    devices: vec![SyncResponseDevice {
                                      id: "123".to_string(),
                                      type_: "action.devices.types.LIGHT".to_string(),
                                      traits: vec!["action.devices.traits.OnOff".to_string()],
                                      name: Name {
                                          default_name: vec!["foo".to_string()],
                                          name: Some("foo".to_string()),
                                          nicknames: vec![],
                                      },
                                      will_report_state: false,
                                      device_info: None,
                                      room_hint: None,
                                      structure_hint: None,
                                  }],
                },
            };
            let res = serde_json::to_string(&response).unwrap_or("".to_string());
            let mut rsp = Response::with((status::Ok, res));
            rsp.headers.set(ContentType::json());
            return Ok(rsp);
        } else if input.intent == "action.devices.QUERY" {
            let response = QueryResponse {
                request_id: action_request.request_id.clone(),
                payload: QueryResponsePayload {
                    devices: btreemap!{
                        "123".to_string() => DeviceStates {
                            online: true,
                            on: true,
                            brightness: 10,
                            color: Color {
                                name: "red".to_string(),
                                temperature: 0,
                                spectrum_rgb: 0,
                            },
                        }
                    },
                },
            };
            let res = serde_json::to_string(&response).unwrap_or("".to_string());
            let mut rsp = Response::with((status::Ok, res));
            rsp.headers.set(ContentType::json());
            return Ok(rsp);
        } else if input.intent == "action.devices.EXECUTE" {
            let response = ExecuteResponse {
                request_id: action_request.request_id.clone(),
                payload: ExecuteResponsePayload {
                    error_code: Some("ERROR".to_string()),
                    debug_string: Some("TODO".to_string()),
                    commands: vec![],
                },
            };
            let res = serde_json::to_string(&response).unwrap_or("".to_string());
            let mut rsp = Response::with((status::Ok, res));
            rsp.headers.set(ContentType::json());
            return Ok(rsp);
        }
    }

    let mut rsp = Response::with((status::Ok, "ACTION"));
    rsp.headers.set(ContentType::json());
    Ok(rsp)
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}
