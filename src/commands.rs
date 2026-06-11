use crate::common::Context;
// use crate::db::entity::macro_model;
use crate::db::entity::prelude::{MacroGroup, Owner};
use anyhow::{Result, anyhow, bail};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, sea_query};
use serenity::all::{AuthorizingIntegrationOwner, InteractionContext};
use crate::db_interface::get_owned_groups;

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
pub async fn r#macro(
    ctx: Context<'_>,
    #[description = "macro name"]
    // #[autocomplete = "poise::builtins::autocomplete_command"]
    name: String,
) -> Result<()> {
    let macros = get_owned_groups(
        &ctx.data().db,
        ctx.author().id.get(),
        ctx.guild_id().map(|g| g.get()),
    )
    .await?;
    dbg!(&macros);
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
enum ContextType {
    User,
    Guild,
}

#[poise::command(slash_command, subcommands("add_macro", "delete", "edit", "group"))]
pub async fn macros(ctx: Context<'_>) -> Result<()> {
    bail!("Dummy parent command shouldn't ever be called directly.")
}

#[poise::command(slash_command, subcommands("add_group"))]
pub async fn group(ctx: Context<'_>) -> Result<()> {
    bail!("Dummy parent command shouldn't ever be called directly.")
}

pub fn can_add_to_guild(ctx: &Context<'_>) -> Result<()> {
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

    // check that the user has manage guild permissions
    if !ctx
        .interaction
        .member
        .clone()
        .ok_or(anyhow!("missing member"))?
        .permissions
        .ok_or(anyhow!("missing permissions"))?
        .manage_guild()
    {
        bail!("You need the Manage Server permission to create guild macros.");
    }
    Ok(())
}

#[poise::command(slash_command, rename = "add")]
pub async fn add_macro(
    ctx: Context<'_>,
    #[description = "The name of the macro"] name: String,
    #[description = "The contents of the macro (text or media *link*)"] contents: String,
    #[description = "The macro group to add to"]
    #[autocomplete = "owned_groups_autocomplete"]
    group: Option<String>,
) -> Result<()> {
    // let owner_id = match context_type {
    //     ContextType::User => ctx.author().id.get(),
    //     ContextType::Guild => {
    //         // check if in guild
    //         can_add_to_guild(&ctx)?;
    //
    //         ctx.guild_id().ok_or(anyhow!("missing guild id"))?.get()
    //     }
    // };

    // let am = db::entity::macro_model::ActiveModel {
    //     owner: Set(owner_id as i64),
    //     name: Set(name),
    //     contents: Set(contents),
    //     ..Default::default()
    // }.insert(&ctx.data().db).await?;

    ctx.say("Added macro!").await?;

    Ok(())
}

#[poise::command(slash_command, rename = "add")]
pub async fn add_group(ctx: Context<'_>, context_type: ContextType, name: String) -> Result<()> {
    let owner_id = match context_type {
        ContextType::User => ctx.author().id.get(),
        ContextType::Guild => {
            can_add_to_guild(&ctx)?;

            ctx.guild_id().ok_or(anyhow!("missing guild id"))?.get()
        }
    };

    // let am = db::entity::macro_model::ActiveModel {
    //     owner: Set(owner_id as i64),
    //     name: Set(name),
    //     contents: Set(contents),
    //     ..Default::default()
    // }.insert(&ctx.data().db).await?;

    dbg!(ctx.author().display_name());
    dbg!(&owner_id);

    let owner = match context_type {
        ContextType::User => Owner::ActiveModel {
            name: Set(ctx.author().name.to_owned()), // TODO does not update name on every insert?
            is_server: Set(false),
            id: Set(owner_id as i64),
            ..Default::default()
        },
        ContextType::Guild => Owner::ActiveModel {
            name: Set(ctx.guild().unwrap().name.to_owned()), // TODO does not update name on every insert?
            is_server: Set(true),
            id: Set(owner_id as i64),
            ..Default::default()
        },
    };

    let nowner = Owner::Entity::insert(owner)
        .on_conflict(
            // on conflict do update
            sea_query::OnConflict::column(Owner::Column::Id)
                .update_column(Owner::Column::Id)
                .to_owned(),
        )
        .exec_with_returning(&ctx.data().db)
        .await?;

    MacroGroup::ActiveModel::builder()
        .set_owner_id(nowner.id)
        .set_owner(nowner.into_active_model())
        .set_name(name.clone())
        .set_is_subscribable(true)
        .save(&ctx.data().db)
        .await?;

    ctx.say(format!(
        "Added macro group `{name}` to {}!",
        match context_type {
            ContextType::User => "user",
            ContextType::Guild => "guild",
        }
    ))
    .await?;

    Ok(())
}

pub async fn owned_groups_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<String> {
    let mut out = vec!();
    if let Ok(grps) = get_owned_groups(
        &ctx.data().db,
        ctx.author().id.get(),
        if can_add_to_guild(&ctx).is_ok() {
            ctx.guild_id().map(|g| g.get())
        } else {
            None
        },
    ).await {
        for (owner, groups) in grps {
            for group in groups {
                out.push(format!("{}/{}", owner.name, group.name))
            }
        }
    }

    out
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
