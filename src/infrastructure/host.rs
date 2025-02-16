use crate::infrastructure::config::Config;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use std::ops::Deref;

pub struct Host {
    host: String,
}

impl Deref for Host {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.host
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Host {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let accessor = request
            .rocket()
            .state::<HostAccessorState>()
            .expect("No HostAccessor registered.");
        Outcome::Success(Self {
            host: accessor.deref().host(request),
        })
    }
}

pub type HostAccessorState = Box<dyn HostAccessor>;

pub trait HostAccessor
where
    Self: Send + Sync + 'static,
{
    fn host(&self, request: &Request<'_>) -> String;
}

pub struct ConfigHostAccessor {}

impl ConfigHostAccessor {
    pub fn new_state() -> HostAccessorState {
        Box::new(ConfigHostAccessor {})
    }
}

impl HostAccessor for ConfigHostAccessor {
    fn host(&self, request: &Request<'_>) -> String {
        request
            .rocket()
            .state::<Config>()
            .expect("Config should be present")
            .host
            .clone()
    }
}

#[cfg(test)]
pub mod test {
    use crate::infrastructure::host::{HostAccessor, HostAccessorState};
    use rocket::Request;

    pub struct StaticHostAccessor {
        host: String,
    }

    impl StaticHostAccessor {
        pub fn new_state(host: String) -> HostAccessorState {
            Box::new(StaticHostAccessor { host })
        }
    }

    impl HostAccessor for StaticHostAccessor {
        fn host(&self, _request: &Request<'_>) -> String {
            self.host.clone()
        }
    }
}
