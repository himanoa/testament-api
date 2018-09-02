use rocket::http::{Cookie, Cookies, Status};
use rocket::response::Redirect;
use uuid::Uuid;
// TODO: Time crate is deprecated.
// Refactoring with a chrono when the cookie crate cut the time crate dependency.
use time::now;
use serde_json::to_value;

use infra::oauth::google::{GoogleProvider};
use infra::oauth::{OAuthProvider};

#[get("/login", format = "application/json")]
pub fn login(mut cookies: Cookies, provider: GoogleProvider) -> Result<Redirect, Status> {
    let mut expire = now();
    expire.tm_hour = expire.tm_hour + 2;
    let uuid = Uuid::new_v4().to_string();
    let (url, state) = match provider.generate_authorize_url(&uuid) {
        Ok((u, s)) => (u,s),
        Err(_) => return Err(Status::ServiceUnavailable)
    };
    let state_cookie = Cookie::build("blog_manage_login_state", state )
        .expires(expire)
        .path("/")
        .secure(true)
        .finish();
    cookies.add(state_cookie);
    Ok(Redirect::to(&url))
}
