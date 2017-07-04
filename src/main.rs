extern crate iron;
extern crate router;
extern crate params;

use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use router::Router;

fn main() {
    println!("Hello, world!");
    let mut control = Router::new();
    control.get("/auth", auth_handler, "auth")
        .get("/token", token_handler, "token")
        .get("/login", login_handler, "login")
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
    Ok(Response::with((status::Ok, "token")))
}

fn login_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "login")))
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "index")))
}
