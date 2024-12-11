use dotenv::dotenv;
use log::debug;
use rocket::launch;
use crate::http::oidc;
use crate::infrastructure::config::Config;
use crate::infrastructure::oidc_client::OidcClient;

mod infrastructure;
mod http;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    dotenv::from_filename(".env.local").ok();
    pretty_env_logger::init_custom_env("CHORES_LOG");
    let config = Config::from_dotenv();

    debug!("Using {:?}", &config);

    rocket::build()
        .configure(Into::<rocket::Config>::into(&config))
        .mount("/oidc", oidc::routes())
        .manage(OidcClient::new(&config).await)
        .manage(config)
}