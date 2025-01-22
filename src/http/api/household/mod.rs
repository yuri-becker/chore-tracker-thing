use rocket::{routes, Route};

mod create;
mod get;
mod response;
mod task;
mod invite;
mod join;

pub fn routes() -> Vec<Route> {
    routes![
        create::create,
        get::get,
        invite::generate_invite,
        join::join,
        task::complete::complete,
        task::create::create,
        task::get::get,
        task::details::details,
    ]
}
