use crate::infrastructure::database::Database;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::{Order, QueryOrder, QuerySelect};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub task_id: Uuid,
    #[sea_orm(primary_key)]
    pub iteration: i32,
    pub due_date: Date,
    pub completed_by: Option<Uuid>,
    pub completed_on: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
    CompletedBy,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Task => Entity::has_one(super::task::Entity).into(),
            Relation::CompletedBy => Entity::has_one(super::user::Entity).into(),
        }
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CompletedBy.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn find_latest(db: &Database, task: Uuid) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(Column::TaskId.eq(task))
        .order_by(Column::Iteration, Order::Desc)
        .limit(1)
        .one(db.conn())
        .await
}
