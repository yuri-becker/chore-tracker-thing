use rocket::{async_trait, Request};
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

pub enum CallbackQuery {
    Error(CallbackError),
    Success(CallbackSuccess),
}
#[derive(Debug)]
pub struct CallbackError {
    pub error: String,
    pub error_description: String,
    pub iss: String,
}

pub struct CallbackSuccess {
    pub code: String,
}

#[derive(Debug)]
pub struct InvalidCallbackError {}

impl From<InvalidCallbackError> for Outcome<CallbackQuery, InvalidCallbackError> {
    fn from(val: InvalidCallbackError) -> Self {
        Outcome::Error((Status::BadRequest, val))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for CallbackQuery {
    type Error = InvalidCallbackError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, InvalidCallbackError> {
        let error = request.query_value("error");
        if error.is_some() {
            let Some(Ok(error)) = error else {
                return InvalidCallbackError {}.into();
            };
            let Some(Ok(error_description)) = request.query_value::<String>("error_description")
            else {
                return InvalidCallbackError {}.into();
            };
            let Some(Ok(iss)) = request.query_value::<String>("iss") else {
                return InvalidCallbackError {}.into();
            };
            Outcome::Success(CallbackQuery::Error(CallbackError {
                error,
                error_description,
                iss,
            }))
        } else {
            let Some(Ok(code)) = request.query_value::<String>("code") else {
                return InvalidCallbackError {}.into();
            };
            Outcome::Success(CallbackQuery::Success(CallbackSuccess { code }))
        }
    }
}