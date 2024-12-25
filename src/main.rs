extern crate core;

use crate::http::api::household;
use crate::http::oidc;
use crate::infrastructure::config::Config;
use crate::infrastructure::database::Database;
use crate::infrastructure::oidc_client::OidcClient;
use dotenv::dotenv;
use log::debug;
use rocket::launch;
use crate::infrastructure::access_control::AccessControl;

mod domain;
mod http;
mod infrastructure;
mod migration;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    dotenv::from_filename(".env.local").ok();
    pretty_env_logger::init_custom_env("CHORES_LOG");
    let config = Config::from_dotenv();
    debug!("Using {:?}", &config);
    let database = Database::connect(&config).await;
    migration::migrate(&database).await;

    rocket::build()
        .attach(AccessControl::new(&config.host))
        .configure(Into::<rocket::Config>::into(&config))
        .mount("/oidc", oidc::routes())
        .mount("/api/household", household::routes())
        .manage(OidcClient::new(&config).await)
        .manage(database)
        .manage(config)
}
