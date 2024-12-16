use std::fmt::{Debug, Formatter};
use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use sea_orm::DbErr;

pub struct DatabaseError {
    err: DbErr,
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        Self { err }
    }
}

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        error!("Caught a database error: {:?}", self.err);
        Err(Status::InternalServerError)
    }
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.err, f)
    }
}