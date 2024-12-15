use crate::domain::user;
use crate::infrastructure::database::Database;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{DeriveEntityModel, TryIntoModel};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "oidc_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub subject: String,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get_or_register(db: &Database, subject: String) -> Result<Model, DbErr> {
    let existing_user = Entity::find_by_id(&subject).one(db.conn()).await?;
    match existing_user {
        None => register(db, subject).await,
        Some(user) => Ok(user),
    }
}

async fn register(db: &Database, subject: String) -> Result<Model, DbErr> {
    let user = user::ActiveModel { id: Set(Uuid::now_v7()) }.insert(db.conn()).await?;
    let oidc_user = ActiveModel {
        subject: Set(subject),
        user_id: Set(user.id),
    }
    .insert(db.conn())
    .await?;
    oidc_user.try_into_model()
}
