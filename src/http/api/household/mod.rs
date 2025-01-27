use rocket::{routes, Route};

mod create;
mod edit;
mod get;
mod invite;
mod join;
mod response;
mod task;

pub fn routes() -> Vec<Route> {
    routes![
        create::create,
        edit::edit,
        get::get,
        invite::generate_invite,
        join::join,
        task::complete::complete,
        task::create::create,
        task::get::get,
        task::details::details,
    ]
}
