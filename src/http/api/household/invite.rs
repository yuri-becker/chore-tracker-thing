use crate::domain::invite;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::UuidParam;
use crate::infrastructure::database::Database;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono::{DateTime, TimeDelta, Utc};
use log::{debug, warn};
use rand::RngCore;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{error, get};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use std::ops::Add;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub invite_code: Uuid,
    pub secret: String,
    pub valid_until: DateTime<Utc>,
}

struct Secret {
    secret: String,
    digest: String,
}

fn generate_secret() -> Result<Secret, argon2::Error> {
    let mut rng = rand::thread_rng();

    let mut secret = [0u8; 16];
    rng.fill_bytes(&mut secret);
    let mut salt = [0u8; 8];
    rng.fill_bytes(&mut salt);

    let digest = argon2::hash_encoded(&secret, &salt, &argon2::Config::default())?;
    let secret = URL_SAFE.encode(secret);
    Ok(Secret { secret, digest })
}

fn validate_secret(secret: &str, digest: &str) -> bool {
    URL_SAFE.decode(secret)
        .inspect_err(|err| debug!("Could not decode secret, given string is likely invalid: {:?}", err))
        .map(|secret| argon2::verify_encoded(digest, secret.as_slice()))
        .unwrap_or(Ok(false))
        .inspect_err(|err| warn!("Failed to decode secret, digest is likely invalid: {:?}", err))
        .unwrap_or(false)
}

#[get("/<household_id>/invite")]
pub async fn get_invite(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
) -> Result<Json<Response>, ApiError> {
    user.in_household(db, *household_id).await?;

    let id = Uuid::now_v7();
    let secret = generate_secret().map_err(|err| {
        error!("Failed to generate invite secret: {:?}", err);
        ApiError::InternalServerError(())
    })?;

    let valid_until = Utc::now().add(TimeDelta::hours(24));
    invite::ActiveModel {
        id: Set(id),
        household_id: Set(*household_id),
        created_by: Set(user.id),
        secret_digest: Set(secret.digest),
        valid_until: Set(valid_until.clone().naive_local())
    }.insert(db.conn()).await.map_err(ApiError::from)?;

    Ok(Json(Response {
        invite_code: id,
        secret: secret.secret,
        valid_until,
    }))
}

#[cfg(test)]
mod test {
    use crate::http::api::household::invite::{generate_secret, validate_secret};

    #[test]
    fn test_secret_round_trip() {
        let secret = generate_secret().unwrap();
        let result = validate_secret(&secret.secret, &secret.digest);
        assert!(result);
    }

    #[test]
    fn test_invalid_secret() {
        let secret = generate_secret().unwrap();
        let result = validate_secret(&secret.digest, (secret.secret + "!").as_str());
        assert!(!result);
    }
}