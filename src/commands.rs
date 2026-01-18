use anyhow::{anyhow, bail, Result};
use sea_orm::{ActiveModelTrait, Set};
use serenity::all::{
    AuthorizingIntegrationOwner, InteractionContext,
};
use crate::common::Context;
use crate::db;

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<()> {
    poise::builtins::help(
        poise::Context::Application(ctx),
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
pub async fn r#macro(
    ctx: Context<'_>,
    #[description = "macro name"]
    // #[autocomplete = "poise::builtins::autocomplete_command"]
    name: String,
) -> Result<()> {
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
enum ContextType {
    User,
    Guild,
}



#[poise::command(slash_command, subcommands("add", "delete", "edit"))]
pub async fn managemacros(ctx: Context<'_>) -> Result<()> {
    bail!("Dummy parent command shouldn't ever be called directly.")
}

#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    context_type: ContextType,
    name: String,
    contents: String,
) -> Result<()> {
    let owner_id = match context_type {
        ContextType::User => ctx.author().id.get(),
        // need to map there because guildref doesnt implement send so async shits itself
        ContextType::Guild => {
            // check if in guild
            if !matches!(
                ctx.interaction.context.ok_or(anyhow!("missing context"))?,
                InteractionContext::Guild
            ) {
                bail!("Command not ran in guild.");
            }

            // check if the bot is installed in the guild
            let aio = &ctx.interaction.authorizing_integration_owners.0;
            if !aio
                .iter()
                .any(|x| matches!(x, AuthorizingIntegrationOwner::GuildInstall(_)))
            {
                bail!("The bot is not installed in this guild.");
            }

            if !ctx.interaction.member.clone().ok_or(anyhow!("missing member"))?.permissions.ok_or(anyhow!("missing permissions"))?.manage_guild() {
                bail!("You need the Manage Server permission to create guild macros.");
            }

            ctx.guild_id().ok_or(anyhow!("missing guild id"))?.get()
        }
    };

    let am = db::entity::macro_model::ActiveModel {
        owner: Set(owner_id as i64),
        name: Set(name),
        contents: Set(contents),
        ..Default::default()
    }.insert(&ctx.data().db).await?;

    ctx.say("Added macro!").await?;

    Ok(())
}

/*match ctx.guild().map(|g| g.clone()) {
Some(guild) => {
    let author_perms = guild.user_permissions_in(
        &(ctx
            .guild_channel()
            .await
            .ok_or(anyhow!("Failed to get guild channel"))?),
        ctx.author_member().await.ok_or(anyhow!("Failed to get author member"))?.to_mut(),
    );
    if !author_perms.manage_guild() {
        bail!("You need the Manage Server permission to create guild macros.");
    }
    guild.id.get()
}
None => {
    bail!("This command must be used in a guild to create guild macros.");
}*/
#[poise::command(slash_command)]
pub async fn delete(ctx: Context<'_>) -> Result<()> {
    bail!("Dummy parent command shouldn't ever be called directly.")
}

#[poise::command(slash_command)]
pub async fn edit(ctx: Context<'_>) -> Result<()> {
    bail!("Dummy parent command shouldn't ever be called directly.")
}

#[poise::command(slash_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
    let reg = crate::common::sync_commands(
        (&ctx).as_ref(),
        ctx.framework().options.commands.as_slice(),
        true,
    )
    .await?;
    ctx.say(format!("{} commands registered!", reg.len()))
        .await?;
    Ok(())
}
