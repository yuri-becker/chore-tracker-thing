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

#[cfg(test)]
mod test {
    use super::AccessControl;
    use crate::test_environment::TestEnvironment;
    use rocket::{async_test, get, routes, uri};

    #[get("/")]
    fn endpoint() -> &'static str {
        "Hello!"
    }

    #[async_test]
    async fn test_allows_from_https_host() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![endpoint])
            .attach(AccessControl::new("https://chores.local"))
            .launch()
            .await;

        let response = env.get(uri!(endpoint)).dispatch().await;
        let allow_origin = response
            .headers()
            .get_one("Access-Control-Allow-Origin")
            .unwrap();
        assert_eq!(allow_origin, "chores.local");
    }

    #[async_test]
    async fn test_allows_from_http_host() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![endpoint])
            .attach(AccessControl::new("http://chores.local"))
            .launch()
            .await;

        let response = env.get(uri!(endpoint)).dispatch().await;
        let allow_origin = response
            .headers()
            .get_one("Access-Control-Allow-Origin")
            .unwrap();
        assert_eq!(allow_origin, "chores.local");
    }
}
