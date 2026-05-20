pub mod entity;
use anyhow::Result;


// static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

use sea_orm::{Database, DatabaseConnection};
// use migration::{Migrator, MigratorTrait};

pub async fn init_db(database_url: String) -> Result<DatabaseConnection> {
    let db = Database::connect(
        database_url,
    )
        .await?;

    // synchronizes database schema with entity definitions
    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    // runs migrations (db stuff i cant do in seaorm)
    // Migrator::up(&db, None).await?;

    // DATABASE.set(db)?;
    Ok(db)
}