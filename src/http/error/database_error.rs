use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use sea_orm::DbErr;
use std::fmt::{Debug, Formatter};

pub struct DatabaseError {
    err: DbErr,
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        Self { err }
    }
}

impl From<DatabaseError> for Status {
    fn from(value: DatabaseError) -> Self {
        error!("Caught a database error: {:?}", value.err);
        Status::InternalServerError
    }
}

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        error!("Caught a database error: {:?}", self.err);
        Err(self.into())
    }
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
