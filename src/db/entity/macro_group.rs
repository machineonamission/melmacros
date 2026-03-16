use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "macro_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub is_subscribable: bool,

    pub name: String,

    // #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(belongs_to)]
    pub owner: HasOne<super::owner::Entity>,

    #[sea_orm(belongs_to)]
    pub created_by: HasOne<super::owner::Entity>,

    #[sea_orm(has_many)]
    pub subscribed: HasMany<super::owner::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
