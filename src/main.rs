extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    println!("Hello, world!");
    let mut control = Router::new();
    control.get("/auth",
             |req: &mut Request| Ok(Response::with((status::Ok, "auth"))),
             "auth")
        .get("/",
             |req: &mut Request| Ok(Response::with((status::Ok, "index"))),
             "index");
    Iron::new(control)
        .http("127.0.0.1:3000")
        .unwrap();
}
