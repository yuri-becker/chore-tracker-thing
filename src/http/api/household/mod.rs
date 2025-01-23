use rocket::{routes, Route};

mod create;
mod get;
mod invite;
mod join;
mod response;
mod task;

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
