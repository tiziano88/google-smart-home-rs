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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SyncResponseDevice {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    name: Name,
    traits: Vec<String>,
    #[serde(rename = "willReportState")]
    will_report_state: bool,
    #[serde(rename = "roomHint")]
    #[serde(skip)]
    room_hint: Option<String>,
    #[serde(rename = "structureHint")]
    #[serde(skip)]
    structure_hint: Option<String>,
    #[serde(rename = "deviceInfo")]
    #[serde(skip)]
    device_info: Option<DeviceInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Name {
    #[serde(rename = "defaultName")]
    #[serde(skip)]
    default_name: Vec<String>,
    name: Option<String>,
    #[serde(skip)]
    nicknames: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct DeviceInfo {
    manifacturer: String,
    model: String,
    #[serde(rename = "hwVersion")]
    hw_version: String,
    #[serde(rename = "swVersion")]
    sw_version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SyncRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<SyncRequestInput>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SyncRequestInput {
    intent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SyncResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: SyncResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct SyncResponsePayload {
    #[serde(rename = "agentUserId")]
    agent_user_id: String,
    devices: Vec<SyncResponseDevice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct QueryRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct QueryResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: QueryResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct QueryResponsePayload {
    devices: BTreeMap<String, DeviceStates>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<ExecuteRequestInput>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteRequestInput {
    intent: String,
    payload: ExecuteRequestInputPayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteRequestInputPayload {
    commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Command {
    devices: Vec<RequestDevice>,
    execution: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct RequestDevice {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Execution {
    command: String,
    params: Params,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Params {
    // TODO: Add more commands.
    on: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct DeviceStates {
    online: bool,
    on: bool,
    brightness: u64,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Color {
    name: String,
    temperature: u64,
    #[serde(rename = "spectrumRGB")]
    spectrum_rgb: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: ExecuteResponsePayload,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteResponsePayload {
    #[serde(rename = "errorCode")]
    error_code: Option<String>,
    #[serde(rename = "debugString")]
    debug_string: Option<String>,
    commands: Vec<ExecuteResponseCommand>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ExecuteResponseCommand {
    ids: Vec<String>,
    status: String,
    states: DeviceStates,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ActionRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<ActionRequestInput>,
}

#[test]
fn test_sync_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.SYNC"
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![ActionRequestInput {
                         intent: "action.devices.SYNC".to_string(),
                         payload: None,
                     }],
    };
    assert_eq!(expected_req, parsed_req);
}

#[test]
fn test_query_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.QUERY",
    "payload": {
      "devices": [{
        "id": "123",
        "customData": {
          "fooValue": 74,
          "barValue": true,
          "bazValue": "foo"
        }
      },{
        "id": "456",
        "customData": {
          "fooValue": 12,
          "barValue": false,
          "bazValue": "bar"
        }
      }]
    }
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![ActionRequestInput {
                         intent: "action.devices.QUERY".to_string(),
                         payload: Some(ActionRequestPayload {
                             devices: vec![RequestDevice { id: "123".to_string() },
                                           RequestDevice { id: "456".to_string() }],
                             commands: vec![],
                         }),
                     }],
    };
    assert_eq!(expected_req, parsed_req);
}


#[test]
fn test_execute_request() {
    let json_req = r#"
{
  "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
  "inputs": [{
    "intent": "action.devices.EXECUTE",
    "payload": {
      "commands": [{
        "devices": [{
          "id": "123",
          "customData": {
            "fooValue": 74,
            "barValue": true,
            "bazValue": "sheepdip"
          }
        },{
          "id": "456",
          "customData": {
            "fooValue": 36,
            "barValue": false,
            "bazValue": "moarsheep"
          }
        }],
        "execution": [{
          "command": "action.devices.commands.OnOff",
          "params": {
            "on": true
          }
        }]
      }]
    }
  }]
}
"#;
    let parsed_req: ActionRequest = serde_json::from_str(&json_req).unwrap();
    let expected_req = ActionRequest {
        request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
        inputs: vec![ActionRequestInput {
                         intent: "action.devices.EXECUTE".to_string(),
                         payload: Some(ActionRequestPayload {
                             devices: vec![],
                             commands: vec![Command {
                                                devices: vec![RequestDevice {
                                                                  id: "123".to_string(),
                                                              },
                                                              RequestDevice {
                                                                  id: "456".to_string(),
                                                              }],
                                                execution: vec![Execution {
                                                                    command: "action.devices.\
                                                                              commands.OnOff"
                                                                        .to_string(),
                                                                    params: Params { on: true },
                                                                }],
                                            }],
                         }),
                     }],
    };
    assert_eq!(expected_req, parsed_req);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ActionRequestInput {
    intent: String,
    payload: Option<ActionRequestPayload>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct ActionRequestPayload {
    #[serde(default)]
    devices: Vec<RequestDevice>,
    #[serde(default)]
    commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct AuthResponse {
    token_type: String,
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

fn main() {
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
