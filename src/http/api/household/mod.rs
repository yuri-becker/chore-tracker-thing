use rocket::{routes, Route};

mod create;
mod get;
mod response;

pub fn routes() -> Vec<Route> {
    routes![create::create, get::get]
}