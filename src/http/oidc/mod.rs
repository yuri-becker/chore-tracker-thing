use rocket::{routes, Route};

mod callback;
mod oidc_error;
mod login;
mod callback_query;
mod logout;

pub fn routes() -> Vec<Route> {
    routes![login::login, callback::callback, logout::logout]
}