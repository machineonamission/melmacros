pub mod entity;
use anyhow::Result;


// static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

use sea_orm::{Database, DatabaseConnection};
pub async fn init_db() -> Result<DatabaseConnection> {
    let db = Database::connect(
        r"sqlite://db.sqlite?mode=rwc",
    )
        .await?;
    // synchronizes database schema with entity definitions
    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    // DATABASE.set(db)?;
    Ok(db)
}