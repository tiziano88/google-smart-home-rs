extern crate iron;
extern crate router;
extern crate params;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use router::Router;

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
    let map = req.get_ref::<Params>().unwrap();

    let redirect_uri = map.find(&["response_type"]);
    let client_id = map.find(&["client_id"]);
    let redirect_uri = map.find(&["redirect_uri"]);
    let scope = map.find(&["scope"]);
    let state = map.find(&["state"]);

    println!("uri: {:?}", redirect_uri);
    println!("scope: {:?}", scope);

    Ok(Response::with((status::Ok, "auth")))
}

fn token_handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

    let grant_type = map.find(&["grant_type"]);
    let code = map.find(&["code"]);
    let redirect_uri = map.find(&["redirect_uri"]);
    let client_id = map.find(&["client_id"]);

    Ok(Response::with((status::Ok, "token")))
}

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

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
