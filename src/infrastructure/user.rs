use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{async_trait, Request};
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoggedInUser {
    pub groups: Vec<String>,
    pub name: String,
}

impl TryFrom<Cookie<'_>> for LoggedInUser {
    type Error = serde_json::Error;

    fn try_from(cookie: Cookie) -> Result<Self, Self::Error> {
        serde_json::from_str::<LoggedInUser>(cookie.value())
    }
}
pub struct User {
    user: Option<LoggedInUser>,
}

impl Deref for User {
    type Target = Option<LoggedInUser>;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.cookies().get_private("user").and_then(|cookie| {
            LoggedInUser::try_from(cookie)
                .map(Some)
                .unwrap_or(None)
        });
        Outcome::Success(User { user })
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = User::from_request(request).await;
        let user = user.expect("User::from_request should always return Success");
        match user.user {
            Some(logged_in_user) => Outcome::Success(logged_in_user),
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
