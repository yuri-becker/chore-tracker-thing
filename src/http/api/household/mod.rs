use rocket::{routes, Route};

mod create;
mod get;
mod response;
mod task;

pub fn routes() -> Vec<Route> {
    routes![
        create::create,
        get::get,
        task::complete::complete,
        task::create::create,
        task::get::get
    ]
}
