// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
mod commands;
mod constants;
mod data;
mod services;

use log;
use serenity::gateway::ActivityData;
use services::env_variables;

use crate::data::Data;

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    initialize();

    log::info!("Starting bot...");

    let options = poise::FrameworkOptions {
        commands: commands::get_commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(env_variables::prefix()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(|ctx| {
            Box::pin(async move {
                let is_bot = ctx.author().bot;
                Ok(!is_bot)
            })
        }),
        skip_checks_for_owners: false,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                log::info!("Bot is ready! Username: {}", ready.user.name);

                log::info!("Registering slash commands...");
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    env_variables::gulid_id(),
                )
                .await?;

                // set game status
                log::info!("Setting game status...");
                ctx.set_activity(Some(ActivityData {
                    name: format!("{}help", env_variables::prefix()).to_string(),
                    kind: serenity::model::gateway::ActivityType::Listening,
                    url: None,
                    state: None,
                }));
                Ok(create_context())
            })
        })
        .options(options)
        .build();
    let intents = serenity::model::gateway::GatewayIntents::all();
    let client = serenity::Client::builder(env_variables::token(), intents)
        .framework(framework)
        .await;

    Ok(client.unwrap().start().await?)
}

fn initialize() {
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Failed to load .env file: {}", e);
    }

    if let Err(_) = std::env::var("RUST_LOG") {
        unsafe {
            std::env::set_var("RUST_LOG", "warning,useless_bot=info");
        }
    }

    pretty_env_logger::init();
}

fn create_context() -> Data {
    Data {}
}

async fn on_error(error: poise::FrameworkError<'_, Data, anyhow::Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            log::error!("Critical error occured: {}", error);
            panic!("Failed to start bot: {:?}", error);
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            log::error!(
                "Error occured while executing command {}: {}",
                ctx.command().name,
                error
            );
        }
        _ => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log::error!("Unexpected error occured: {}", e)
            }
        }
    }
}
