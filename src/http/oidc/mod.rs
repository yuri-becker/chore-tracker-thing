use rocket::{routes, Route};

mod callback;
mod callback_query;
mod login;
mod logout;
mod oidc_error;

pub fn routes() -> Vec<Route> {
    routes![login::login, callback::callback, logout::logout]
}
