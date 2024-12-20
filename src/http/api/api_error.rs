use log::error;
use rocket::Responder;
use sea_orm::DbErr;

#[derive(Responder)]
pub enum ApiError {
    #[response(status = 403)]
    NotInHousehold(()),
    #[response(status = 422)]
    InvalidRequest(&'static str),
    #[response(status = 500)]
    DatabaseError(()),
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        error!("Caught a database error: {:?}", value);
        Self::DatabaseError(())
    }
}
