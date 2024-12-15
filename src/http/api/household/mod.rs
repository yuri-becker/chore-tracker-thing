use rocket::{routes, Route};

mod create;
pub fn routes() -> Vec<Route> {
    routes![create::create]
}