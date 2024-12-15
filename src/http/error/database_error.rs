use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use sea_orm::DbErr;

pub struct DatabaseError {}

impl From<DbErr> for DatabaseError {
    fn from(_: DbErr) -> Self {
        Self {}
    }
}

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        Err(Status::InternalServerError)
    }
}
