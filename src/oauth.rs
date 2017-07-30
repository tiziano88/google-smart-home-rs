extern crate iron;
extern crate params;
extern crate serde_json;

use iron::middleware::Handler;
use iron::modifiers::Redirect;
use iron::prelude::{IronResult, Request, Response};
use iron::prelude::*;
use iron::status;
use url::Url;

use google_actions::*;

pub struct OAuth {
    pub auth: OAuthAuth,
    pub token: OAuthToken,
    pub login: OAuthLogin,
}

impl OAuth {
    pub fn new() -> OAuth {
        OAuth {
            auth: OAuthAuth {},
            token: OAuthToken {},
            login: OAuthLogin {},
        }
    }
}

pub struct OAuthAuth {}

impl Handler for OAuthAuth {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
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
}

pub struct OAuthToken {}

impl Handler for OAuthToken {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
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
}

pub struct OAuthLogin {}

impl Handler for OAuthLogin {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let map = req.get_ref::<params::Params>().unwrap();

        let _ = map.find(&["username"]);
        let _ = map.find(&["password"]);

        Ok(Response::with((status::Ok, "login")))
    }
}
