use rocket::{routes, Route};

mod task;
mod create;
mod get;
mod response;

pub fn routes() -> Vec<Route> {
    routes![create::create, get::get, task::create::create]
}
