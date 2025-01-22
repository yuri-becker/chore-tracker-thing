use log::debug;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{async_trait, Request, Response};

pub struct AccessControl {
    host: String,
}

impl AccessControl {
    pub fn new(host: &str) -> Self {
        let host = host
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .trim_end_matches("/")
            .to_string();
        debug!("Restricting Access-Control-Allowed-Origin to: {}", host);
        Self { host }
    }
}

#[async_trait]
impl Fairing for AccessControl {
    fn info(&self) -> Info {
        Info {
            name: "Sets Access-Control-Allow-Origin header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            self.host.clone(),
        ));
    }
}
