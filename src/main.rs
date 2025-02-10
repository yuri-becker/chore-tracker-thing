extern crate core;

use crate::http::api::guards::logged_in_user::OidcLoggedInUserResolver;
use crate::http::api::household;
use crate::http::oidc;
use crate::infrastructure::access_control::AccessControl;
use crate::infrastructure::config::Config;
use crate::infrastructure::database::Database;
use crate::infrastructure::oidc_client::OidcClient;
use dotenv::dotenv;
use log::debug;
use rocket::launch;

mod domain;
mod http;
mod infrastructure;
mod migration;
#[cfg(test)]
mod test_environment;

#[cfg(not(tarpaulin_include))]
#[launch]
async fn rocket() -> _ {
    init_dotenv();
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
        .manage(OidcClient::from_config(&config).await)
        .manage(OidcLoggedInUserResolver::new_state())
        .manage(infrastructure::host::ConfigHostAccessor::new_state())
        .manage(database)
        .manage(config)
}

pub fn init_dotenv() {
    dotenv::from_filename(".env.local").ok();
    dotenv().ok();
}
