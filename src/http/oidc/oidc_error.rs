use rocket::Responder;

#[derive(Responder)]
pub enum OidcError {
    #[response(status = 401)]
    Unauthorized(()),
    #[response(status = 500)]
    Misconfiguration(()),
    #[response(status = 500)]
    OidcEndpointUnreachable(()),
}