mod db;

mod commands;
mod config;
mod common;

use anyhow::{Error, Result};

use poise::serenity_prelude as serenity;
use common::Data;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    if let Err(e) = poise::builtins::on_error(error).await {
        println!("Error while handling error: {}", e)
    }
    // match error {
    //     poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
    //     poise::FrameworkError::Command { error, ctx, .. } => {
    //         println!("Error in command `{}`: {:?}", ctx.command().name, error,);
    //     }
    //     error => {
    //         if let Err(e) = poise::builtins::on_error(error).await {
    //             println!("Error while handling error: {}", e)
    //         }
    //     }
    // }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::register(), commands::r#macro(), commands::macros()],
        // prefix_options: poise::PrefixFrameworkOptions {
        //     prefix: None,
        //     mention_as_prefix: true,
        //     ..Default::default()
        // },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        // command_check: Some(|ctx| {
        //     Box::pin(async move {
        //         if ctx.author().id == 123456789 {
        //             return Ok(false);
        //         }
        //         Ok(true)
        //     })
        // }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        // skip_checks_for_owners: false,
        // event_handler: |_ctx, event, _framework, _data| {
        //     Box::pin(async move {
        //         println!(
        //             "Got an event in event handler: {:?}",
        //             event.snake_case_name()
        //         );
        //         Ok(())
        //     })
        // },
        ..Default::default()
    };

    let config = config::load_config().await?;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);

                common::sync_commands(ctx, framework.options().commands.as_slice(), false).await?;
                Ok(Data {
                    db: db::init_db().await?,
                })
            })
        })
        .options(options)
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await;

    client?.start().await?;
    Ok(())
}
