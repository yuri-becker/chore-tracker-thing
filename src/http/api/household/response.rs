use crate::domain::{household, household_member, user};
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use rocket::async_trait;
use rocket::futures::future::try_join_all;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DbErr, ModelTrait};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Member {
    id: Uuid,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Household {
    pub id: Uuid,
    pub name: String,
    pub members: Vec<Member>,
}

#[async_trait]
impl FromModel<household::Model> for Household {
    async fn from_model(db: &Database, value: household::Model) -> Result<Self, DbErr> {
        let members = value
            .find_related(household_member::Entity)
            .all(db.conn())
            .await?;
        let members = members
            .iter()
            .map(|it| it.find_related(user::Entity).one(db.conn()))
            .collect::<Vec<_>>();
        let members = try_join_all(members).await?;
        Ok(Self {
            id: value.id,
            name: value.name,
            members: members.iter().map(move |member| {
                let member = member.clone().expect("household_member without user should be prevented by foreign key constraints");
                Member {
                    id: member.id,
                    name: member.name()
                }
            }).collect(),
        })
    }
}
