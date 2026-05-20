use crate::db::entity::owner::ModelEx;
use crate::db::entity::prelude::*;
use anyhow::Result;
use sea_orm::{DatabaseConnection, EntityLoaderTrait};

pub async fn get_available_macros(
    db: &DatabaseConnection,
    owner_id: u64,
    server_id: Option<u64>,
) -> Result<Vec<ModelEx>> {
    let mut macros: Vec<ModelEx> = vec![];
    // apparently flatten() expands the Some() shit, according to fuckin uhh clippy
    for id in [Some(owner_id), server_id].iter().flatten() {
        macros.extend(
            Owner::Entity::load()
                // get owner
                .filter_by_id(*id as i64)
                // many-many jump to groups
                .with((MacroGroup::Entity, Macro::Entity))
                .all(db)
                .await?,
        );
    }

    Ok(macros)
}

pub async fn get_available_groups(
    db: &DatabaseConnection,
    owner_id: u64,
    server_id: Option<u64>,
) -> Result<Vec<ModelEx>> {
    let mut macros: Vec<ModelEx> = vec![];
    // apparently flatten() expands the Some() shit, according to fuckin uhh clippy
    for id in [Some(owner_id), server_id].iter().flatten() {
        macros.extend(
            Owner::Entity::load()
                // get owner
                .filter_by_id(*id as i64)
                .with(MacroGroup::Entity)
                .all(db)
                .await?,
        );
    }

    Ok(macros)
}


// pub async fn get_or_make_default_group(db: &DatabaseConnection, owner: u64) {
//     MacroGroup::ActiveModel::builder()
//         .set_owner(
//             Owner::ActiveModel::builder()
//                 .set_name()
//                 .
//         ).save(db)
// }
//
// pub async fn add_macro(
//     db: &DatabaseConnection,
//     name: String,
//     contents: String) {}