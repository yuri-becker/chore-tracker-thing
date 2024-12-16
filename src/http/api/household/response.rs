use crate::domain::{household, household_member};
use crate::infrastructure::database::Database;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DbErr, ModelTrait};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Household {
    pub id: Uuid,
    pub name: String,
    pub members: Vec<Uuid>,
}

impl Household {
    pub async fn from_model(db: &Database, value: household::Model) -> Result<Self, DbErr> {
        let members = value
            .find_related(household_member::Entity)
            .all(db.conn())
            .await?;
        Ok(Self {
            id: value.id,
            name: value.name,
            members: members.iter().map(|it| it.user_id).collect(),
        })
    }
}
