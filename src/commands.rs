use crate::{Context, Error};
use anyhow::Result;

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            // extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<()> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
    let reg = crate::sync_commands((&ctx).as_ref(), ctx.framework().options.commands.as_slice(), true).await?;
    ctx.say(format!("{} commands registered!", reg.len())).await?;
    Ok(())
}