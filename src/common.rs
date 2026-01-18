use sea_orm::DatabaseConnection;
use anyhow::Error;

// Types used by all command functions
pub type Context<'a> = poise::ApplicationContext<'a, Data, Error>;


// Custom user data passed to all command functions
pub struct Data {
    pub db: DatabaseConnection,
}

pub async fn sync_commands(
    ctx: &serenity::all::Context,
    cmd_list: &[poise::Command<Data, Error>],
    force: bool,
) -> anyhow::Result<Vec<serenity::all::Command>> {
    let create_commands = poise::builtins::create_application_commands(cmd_list);

    let existing_commands = serenity::all::Command::get_global_commands(ctx).await?;

    if existing_commands.len() != create_commands.len() || force {
        println!("Syncing {} commands...", create_commands.len());
        Ok(serenity::all::Command::set_global_commands(ctx, create_commands).await?)
    } else {
        println!("Not syncing commands. run /sync to force.");
        Ok(existing_commands)
    }
}