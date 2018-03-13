extern crate rocket;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Json;
use url::Url;

#[derive(FromForm, Debug)]
struct AuthForm {
    response_type: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

#[get("/auth?<data>")]
fn auth(data: AuthForm) -> Redirect {
    debug!("auth data: {:?}", data);

    let s = match data.state {
        Some(x) => x,
        _ => "".to_string(),
    };

    let u = match data.redirect_uri {
        Some(x) => {
            let mut url = Url::parse(&x).unwrap();
            url.set_query(Some(&format!("code=123&state={}", s)));
            url
        }
        _ => Url::parse("").unwrap(),
    };

    Redirect::found(u.as_str())
}

#[derive(FromForm)]
struct TokenForm {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    token_type: String,
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

#[post("/token", data = "<data>")]
fn token(data: Form<TokenForm>) -> Json<AuthResponse> {
    Json(AuthResponse {
        token_type: "bearer".to_string(),
        access_token: "xxx".to_string(),
        refresh_token: "yyy".to_string(),
        expires_in: 1000000,
    })
}

#[get("/login")]
fn login() -> String {
    "login".to_string()
}
