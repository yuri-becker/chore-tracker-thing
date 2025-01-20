use rocket::{routes, Route};

mod create;
mod get;
mod response;
mod task;
mod invite;

pub fn routes() -> Vec<Route> {
    routes![
        create::create,
        get::get,
        invite::get_invite,
        task::complete::complete,
        task::create::create,
        task::get::get,
        task::details::details,
    ]
}
