use rocket::{routes, Route};

mod create;
mod edit;
mod get;
mod invite;
mod join;
mod leave;
mod response;
mod task;

pub fn routes() -> Vec<Route> {
    routes![
        create::create,
        edit::edit,
        get::get,
        invite::generate_invite,
        join::join,
        leave::leave,
        task::complete::complete,
        task::create::create,
        task::edit::edit,
        task::get::get,
        task::details::details,
    ]
}
