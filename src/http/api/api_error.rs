use log::error;
use rocket::serde::json::Json;
use rocket::Responder;
use sea_orm::{DbErr, TransactionError};

#[derive(Responder)]
pub enum ApiError {
    #[response(status = 403)]
    NotInHousehold(()),
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 409)]
    Conflict(()),
    #[response(status = 422)]
    InvalidRequest(&'static str),
    #[response(status = 500)]
    DatabaseError(()),
    #[response(status = 500)]
    InternalServerError(()),
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        error!("Caught a database error: {:?}", value);
        Self::DatabaseError(())
    }
}

impl From<TransactionError<DbErr>> for ApiError {
    fn from(value: TransactionError<DbErr>) -> Self {
        error!("Caught a database error in a transaction: {:?}", value);
        Self::DatabaseError(())
    }
}

pub type EmptyApiResult = Result<(), ApiError>;
pub type ApiResult<T> = Result<Json<T>, ApiError>;
