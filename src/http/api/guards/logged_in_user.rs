use crate::domain::{household_member, oidc_user, user};
use crate::http::api::api_error::{ApiError, EmptyApiResult};
use crate::infrastructure::database::Database;
use crate::infrastructure::oidc_client::OidcClient;
use log::{debug, warn};
use openid::Jws;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct LoggedInUser {
    pub id: Uuid,
}

impl LoggedInUser {
    pub async fn in_household(&self, database: &Database, household_id: Uuid) -> EmptyApiResult {
        household_member::Entity::find()
            .filter(household_member::Column::HouseholdId.eq(household_id))
            .filter(household_member::Column::UserId.eq(self.id))
            .one(database.conn())
            .await
            .map_err(ApiError::from)?
            .map(|_| ())
            .ok_or(ApiError::NotInHousehold(()))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = Status;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let resolver = request
            .rocket()
            .state::<LoggedInUserResolverState>()
            .expect("No LoggedInUserResolver registered.");
        match resolver.resolve(request).await {
            Ok(user) => Outcome::Success(user),
            Err(status) => Outcome::Error((status, status)),
        }
    }
}

impl From<user::Model> for LoggedInUser {
    fn from(val: user::Model) -> Self {
        LoggedInUser { id: val.id }
    }
}

impl From<oidc_user::Model> for LoggedInUser {
    fn from(val: oidc_user::Model) -> Self {
        LoggedInUser { id: val.user_id }
    }
}

pub type LoggedInUserResolverState = Box<dyn LoggedInUserResolver>;

#[async_trait]
pub trait LoggedInUserResolver
where
    Self: Send + Sync + 'static,
{
    async fn resolve(&self, request: &Request) -> Result<LoggedInUser, Status>;
}

pub struct OidcLoggedInUserResolver {}

impl OidcLoggedInUserResolver {
    pub fn new_state() -> LoggedInUserResolverState {
        Box::new(OidcLoggedInUserResolver {})
    }
}

#[async_trait]
impl LoggedInUserResolver for OidcLoggedInUserResolver {
    async fn resolve(&self, request: &Request) -> Result<LoggedInUser, Status> {
        let database = request
            .rocket()
            .state::<Database>()
            .expect("Database is not in State, this should not occur.");
        let oidc_client = request
            .rocket()
            .state::<OidcClient>()
            .expect("OIDC_Client is not in State, this should not occur.");
        let token = request.cookies().get_private("oidc_token");
        let token = token.ok_or(Status::Unauthorized).inspect_err(|_| {
            debug!("No token found in cookie.");
        })?;
        let mut token = Jws::new_encoded(token.value());
        oidc_client.decode_token(&mut token).map_err(|err| {
            warn!("Could not decode token: {}", err);
            Status::Unauthorized
        })?;
        oidc_client
            .validate_token(&token, None, None)
            .map_err(|err| {
                debug!("Failed to validate token: {}", err);
                Status::Unauthorized
            })?;
        let payload = token.payload().map_err(|_| {
            debug!("Token does not have payload");
            Status::Unauthorized
        })?;
        let user = oidc_user::Entity::find_by_id(&payload.sub)
            .one(database.conn())
            .await
            .map_err(|err| {
                warn!("Could not validate user due to Database err: {}", err);
                Status::InternalServerError
            })?;
        let user = user.ok_or(Status::Unauthorized)?;
        Ok(user.into())
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::api::api_error::ApiError;
    use crate::http::api::guards::logged_in_user::{
        LoggedInUser, LoggedInUserResolver, LoggedInUserResolverState,
    };
    use crate::http::api::UuidParam;
    use crate::infrastructure::database::Database;
    use crate::migration::async_trait::async_trait;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::http::Status;
    use rocket::{async_test, get, routes, Request, State};
    use uuid::Uuid;

    pub struct TestLoggedInUserResolver {}

    impl TestLoggedInUserResolver {
        pub fn new_state() -> LoggedInUserResolverState {
            Box::new(TestLoggedInUserResolver {})
        }
    }

    #[async_trait]
    impl LoggedInUserResolver for TestLoggedInUserResolver {
        async fn resolve(&self, request: &Request) -> Result<LoggedInUser, Status> {
            let user = request.headers().get_one("X-Test-User");
            let user = user.ok_or(Status::Unauthorized)?;
            Ok(LoggedInUser {
                id: Uuid::parse_str(user).unwrap(),
            })
        }
    }

    #[get("/<household_id>")]
    async fn endpoint(
        user: LoggedInUser,
        db: &State<Database>,
        household_id: UuidParam,
    ) -> Result<&'static str, ApiError> {
        user.in_household(db, *household_id).await?;
        Ok("Hello")
    }
    #[async_test]
    async fn test_throws_unauthorized_when_no_user_given() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![endpoint])
            .launch()
            .await;

        let response = env.get(format!("/{}", Uuid::now_v7())).dispatch().await;
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[async_test]
    async fn test_in_household_throws_forbidden_when_not_in_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![endpoint])
            .launch()
            .await;

        let household = env.create_household(None, TestUser::B).await;

        let response = env
            .get(format!("/{}", household.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Forbidden);
    }

    #[async_test]
    async fn test_passes_when_in_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![endpoint])
            .launch()
            .await;

        let household = env
            .create_household(Some("My Household"), TestUser::A)
            .await;
        let response = env
            .get(format!("/{}", household.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }
}
