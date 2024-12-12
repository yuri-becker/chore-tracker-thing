use crate::domain::oidc_user;
use crate::infrastructure::database::Database;
use crate::infrastructure::oidc_client::OidcClient;
use log::{debug, warn};
use openid::Jws;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use sea_orm::EntityTrait;

pub struct LoggedInUser {
    pub id: i32,
}

#[async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = Status;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match from_req(request).await {
            Ok(user) => Outcome::Success(user),
            Err(status) => Outcome::Error((status, status)),
        }
    }
}
async fn from_req(request: &Request<'_>) -> Result<LoggedInUser, Status> {
    let database = request
        .rocket()
        .state::<Database>()
        .expect("Database is not in State, this should not occur.");
    let oidc_client = request
        .rocket()
        .state::<OidcClient>()
        .expect("OIDC_Client is not in State, this should not occur.");
    let token = request.cookies().get_private("oidc_token");
    let token = token.ok_or(Status::Unauthorized).inspect_err(|_| {
        debug!("No token found in cookie.");
    })?;
    let mut token = Jws::new_encoded(token.value());
    oidc_client.decode_token(&mut token).map_err(|err| {
        warn!("Could not decode token: {}", err);
        Status::Unauthorized
    })?;
    oidc_client
        .validate_token(&token, None, None)
        .map_err(|err| {
            debug!("Failed to validate token: {}", err);
            Status::Unauthorized
        })?;
    let payload = token.payload().map_err(|_| {
        debug!("Token does not have payload");
        Status::Unauthorized
    })?;
    let user = oidc_user::Entity::find_by_id(&payload.sub)
        .one(database.conn())
        .await
        .map_err(|err| {
            warn!("Could not validate user due to Database err: {}", err);
            Status::InternalServerError
        })?;
    let user = user.ok_or(Status::Unauthorized)?;
    Ok(LoggedInUser { id: user.user_id })
}
