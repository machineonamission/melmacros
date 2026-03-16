use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "owner")]
pub struct Model {
    // server or user id
    #[sea_orm(primary_key)]
    pub discord_id: i64,

    pub is_user: bool,  // false means is server

    pub name: String,
}

impl ActiveModelBehavior for ActiveModel {}
