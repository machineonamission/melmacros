pub mod entity;
use anyhow::Result;


// static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub async fn init_db() -> Result<DatabaseConnection> {
    let db = Database::connect(
        r"sqlite://melmacros.db?mode=rwc",
    )
        .await?;

    // synchronizes database schema with entity definitions
    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    // runs migrations (db stuff i cant do in seaorm)
    Migrator::up(&db, None).await?;

    // DATABASE.set(db)?;
    Ok(db)
}