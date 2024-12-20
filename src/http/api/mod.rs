use crate::infrastructure::database::Database;
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
            .map_err(|_| Status::BadRequest)
            .map(UuidParam)
    }
}
