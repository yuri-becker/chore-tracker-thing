use crate::infrastructure::user::LoggedInUser;
use rocket::{get, routes, Route};

#[get("/self")]
fn get_self(user: LoggedInUser) -> String {
    user.id.to_string()
}

pub fn routes() -> Vec<Route> {
    routes![get_self]
}
