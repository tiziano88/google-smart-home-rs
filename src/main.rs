extern crate iron;
extern crate router;
extern crate params;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::io::Read;

use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status;
use router::Router;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Fulfillment {
    conversation_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Intent {
    name: String,
    parameters: Vec<Parameter>,
    trigger: Trigger,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trigger {
    query_patterns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameter {
    name: String,
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Action {
    name: String,
    fulfillment: Fulfillment,
    intent: Intent,
    description: String,
    #[serde(rename = "signInRequired")]
    sign_in_required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Type {
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionPackage {
    actions: Vec<Action>,
    types: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncResponseDevice {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    traits: Vec<String>,
    #[serde(rename = "willReportState")]
    will_report_state: bool,
    #[serde(rename = "roomHint")]
    room_hint: Option<String>,
    #[serde(rename = "structureHint")]
    structure_hint: Option<String>,
    config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    manifacturer: String,
    model: String,
    #[serde(rename = "hwVersion")]
    hw_version: String,
    #[serde(rename = "swVersion")]
    sw_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<SyncRequestInput>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncRequestInput {
    intent: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: SyncResponsePayload,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncResponsePayload {
    #[serde(rename = "agentUserId")]
    agent_user_id: String,
    devices: Vec<SyncResponseDevice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryRequest {}

#[derive(Serialize, Deserialize, Debug)]
struct QueryResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: QueryResponsePayload,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryResponsePayload {
    devices: BTreeMap<String, DeviceStates>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<ExecuteRequestInput>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteRequestInput {
    intent: String,
    payload: ExecuteRequestInputPayload,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteRequestInputPayload {
    commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    devices: Vec<RequestDevice>,
    execution: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestDevice {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Execution {
    command: String,
    params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    // TODO: Add more commands.
    on: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceStates {
    online: bool,
    on: bool,
    brightness: u64,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    name: String,
    temperature: u64,
    #[serde(rename = "spectrumRGB")]
    spectrum_rgb: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: ExecuteResponsePayload,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteResponsePayload {
    commands: Vec<ExecuteResponseCommand>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExecuteResponseCommand {
    ids: Vec<String>,
    status: String,
    states: DeviceStates,
}

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionRequestInput {
    intent: String,
    payload: Option<ActionRequestPayload>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionRequestPayload {
    #[serde(default)]
    devices: Vec<RequestDevice>,
    #[serde(default)]
    commands: Vec<Command>,
    #[serde(default)]
    execution: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Debug)]
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
        .get("/token", token_handler, "token")
        .get("/login", login_handler, "login")
        .post("/action", action_handler, "action")
        .get("/", index_handler, "index");
    Iron::new(control)
        .http("127.0.0.1:3000")
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

    let u = match redirect_uri {
        Some(&params::Value::String(ref x)) => {
            let mut url = Url::parse(&x).unwrap();
            url.set_query(Some("code=123"));
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
    let action_package: ActionPackage = serde_json::from_str(&body).unwrap();
    println!("action_package: {:?}", action_package);
    Ok(Response::with((status::Ok, "index")))
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}
