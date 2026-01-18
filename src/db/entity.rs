use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "macro")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    // server or user id
    #[sea_orm(unique_key="name")]
    pub owner: i64,

    #[sea_orm(unique_key="name")]
    pub name: String,

    pub contents: String,
}

impl ActiveModelBehavior for ActiveModel {}
