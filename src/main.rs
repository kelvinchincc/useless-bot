// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
mod commands;
mod constants;
mod errors;
mod services;
mod types;
mod utils;

use anyhow::Result;
use log;
use serenity::gateway::ActivityData;
use services::env_variables;

use types::data::Data;

#[tokio::main]
async fn main() -> Result<()> {
    initialize();

    log::info!("Starting bot...");

    let options = create_framework_options();
    let framework = create_framework(options);
    let intents = serenity::model::gateway::GatewayIntents::all();
    let client = serenity::Client::builder(env_variables::token()?, intents)
        .framework(framework.await?)
        .await;

    match handle_graceful_shutdown(&client) {
        Ok(_) => log::info!("Graceful shutdown handler set up successfully"),
        Err(e) => log::warn!("Failed to set up graceful shutdown handler: {}", e),
    }

    client?.start().await.map_err(|e| {
        log::error!("Failed to start client: {:#}", e);
        anyhow::anyhow!("Failed to start client: {:#}", e)
    })
}

fn initialize() {
    dotenv::dotenv()
        .map_err(|e| log::warn!("Failed to load .env file: {:#}", e))
        .ok();

    if let Err(_) = std::env::var("RUST_LOG") {
        unsafe {
            std::env::set_var("RUST_LOG", "warning,useless_bot=info");
        }
    }

    env_logger::init();
}

async fn create_context() -> Result<Data> {
    use services::config_reader::parse_keyword_response_config;
    use utils::link_helper::get_current_curl_version;

    Ok(Data {
        keyword_response_dict: parse_keyword_response_config()
            .map_err(|e| {
                log::warn!("Failed to read keyword response config due to: {:#}", e);
            })
            .unwrap_or_default(),
        curl_user_agent: get_current_curl_version().await?,
    })
}

async fn on_error(error: poise::FrameworkError<'_, Data, anyhow::Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            log::error!("Critical error occured: {:#}", error);
            panic!("Failed to start bot: {:?}", error);
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            log::error!(
                "Error occured while executing command {}: {:#}",
                ctx.command().name,
                error
            );
        }
        _ => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log::error!("Unexpected error occured: {:#}", e)
            }
        }
    }
}

fn create_framework_options() -> poise::FrameworkOptions<Data, anyhow::Error> {
    poise::FrameworkOptions {
        commands: commands::get_commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(env_variables::prefix()),
            ..Default::default()
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(async move {
                services::events::handle_events(ctx, event, &framework, data).await?;
                Ok(())
            })
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
    }
}

async fn create_framework(
    options: poise::FrameworkOptions<Data, anyhow::Error>,
) -> Result<poise::Framework<Data, anyhow::Error>> {
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                log::info!("Registering slash commands...");
                let use_guild_commands = env_variables::use_guild_commands();
                if use_guild_commands {
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        env_variables::gulid_id()?,
                    )
                    .await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                // set game status
                log::info!("Setting game status...");
                ctx.set_activity(Some(ActivityData {
                    name: format!("{}hi", env_variables::prefix()).to_string(),
                    kind: serenity::model::gateway::ActivityType::Listening,
                    url: None,
                    state: None,
                }));
                Ok(create_context().await?)
            })
        })
        .options(options)
        .build();

    Ok(framework)
}

fn handle_graceful_shutdown(client: &Result<serenity::Client, serenity::Error>) -> Result<()> {
    // Gracefully shutdown the bot on Ctrl+C
    let result = match client {
        Ok(c) => {
            let shard_manager = c.shard_manager.clone();
            tokio::spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .map_err(|_| log::error!("Failed to listen for Ctrl+C signal"))
                    .unwrap();
                log::info!("Received Ctrl+C, shutting down...");
                shard_manager.shutdown_all().await;
            });
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to create client: {:#}", e);
            Err(e)
        }
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(
            "Failed to set up graceful shutdown handler: {:#}",
            e
        )),
    }
}
