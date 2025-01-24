use crate::infrastructure::database::Database;
use log::debug;
use rocket::async_trait;
use rocket::http::Status;
use rocket::request::FromParam;
use sea_orm::{DbErr, ModelTrait};
use std::ops::Deref;
use uuid::Uuid;

mod api_error;
pub mod guards;
pub mod household;

#[async_trait]
pub trait FromModel<TModel: ModelTrait>
where
    Self: Sized,
{
    async fn from_model(db: &Database, model: TModel) -> Result<Self, DbErr>;
}

#[derive(Debug)]
struct UuidParam(Uuid);

impl Deref for UuidParam {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromParam<'_> for UuidParam {
    type Error = Status;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
            .map_err(|err| {
                debug!("Could not parse UUID: {}", err);
                Status::BadRequest
            })
            .map(UuidParam)
    }
}

#[cfg(test)]
mod test {
    use crate::http::api::UuidParam;
    use rocket::http::Status;
    use rocket::request::FromParam;

    #[test]
    fn test_throws_bad_request_when_invalid_uuid() {
        let err = UuidParam::from_param("no-uuid").expect_err("Should have errored.");
        assert_eq!(err, Status::BadRequest);
    }

    #[test]
    fn test_returns_uuid() {
        let uuid = UuidParam::from_param("0194983b-debf-777e-8646-7b0739c208b8");
        assert!(uuid.is_ok());
    }
}
