use crate::domain::user;
use crate::infrastructure::database::Database;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{DeriveEntityModel, IntoActiveModel, TryIntoModel};

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

pub async fn get_or_register(
    db: &Database,
    subject: String,
    display_name: String,
) -> Result<user::Model, DbErr> {
    let existing_user = Entity::find_by_id(&subject)
        .find_also_related(user::Entity)
        .one(db.conn())
        .await?;
    match existing_user {
        None => register(db, subject, display_name).await,
        Some(user) => {
            update_display_name(
                db,
                user.1.expect(
                    "oidc_user without user should be prevented by foreign key constraint.",
                ),
                display_name,
            )
            .await
        }
    }
}

async fn register(
    db: &Database,
    subject: String,
    display_name: String,
) -> Result<user::Model, DbErr> {
    let user = user::ActiveModel {
        display_name: Set(Some(display_name)),
        ..user::ActiveModel::new()
    }
    .insert(db.conn())
    .await?;
    ActiveModel {
        subject: Set(subject),
        user_id: Set(user.id),
    }
    .insert(db.conn())
    .await?;
    user.try_into_model()
}

async fn update_display_name(
    db: &Database,
    user: user::Model,
    display_name: String,
) -> Result<user::Model, DbErr> {
    if user
        .display_name
        .clone()
        .map_or(false, |it| it.eq(&display_name))
    {
        Ok(user)
    } else {
        let mut user = user.into_active_model();
        user.display_name = Set(Some(display_name));
        user.save(db.conn()).await?.try_into_model()
    }
}
