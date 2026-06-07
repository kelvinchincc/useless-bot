// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{constants, services::env_variables, types::context};
use anyhow::Result;
use poise::CreateReply;
use std::default::Default;

/// Greeting to the triggerer, with a banner and link to the github repository.
///
/// This command is designed to provide users with a friendly introduction to the bot, including a banner image and a link to the bot's GitHub repository. When a user invokes the `hi` command, the bot will respond with an embedded message that includes a greeting, a brief description of the bot's purpose, and a banner image. Additionally, the embedded message will contain a hyperlink to the bot's GitHub repository, allowing users to easily access the source code and contribute if they wish. This command serves as a welcoming entry point for users who are new to the bot and want to learn more about it.
#[poise::command(prefix_command)]
pub async fn hi(ctx: context::Context<'_>) -> Result<()> {
    let bot = ctx.framework().bot_id;
    let name = match ctx.guild() {
        Some(gulid) => gulid
            .members
            .get(&bot)
            .map(|member| member.display_name().to_string()),
        None => ctx.cache().user(bot).map(|user| user.name.clone()),
    };

    let embed = serenity::builder::CreateEmbed::default()
        .title(env_variables::greeting())
        .description(format!(concat!(
            "{} is a bot based on [useless-bot]({}). ",
            "This bot have no idea what it can do nor what it will do, it is just created for exporing how a discord ",
            "bot can do or what it can achive.\n\nNot sure where to start? Try type `{}help`!"
        ), name.unwrap_or("Useless Bot".to_string()), constants::PROJECT_GITHUB_URL, env_variables::prefix()))
        .image(constants::PROJECT_BANNER_URL)
        .url(constants::PROJECT_GITHUB_URL)
        // 0x010409
        .color((1, 4, 9));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}

/// Pong!
///
/// A simple command to test if the bot is responsive. When you use the `ping` command, the bot will reply with `Pong!`, indicating that it is online and able to respond to commands. This is a common command used in many bots to check their responsiveness and connectivity.
#[poise::command(prefix_command)]
pub async fn ping(ctx: context::Context<'_>) -> Result<()> {
    ctx.say(":ping_pong: Pong!").await?;
    Ok(())
}

/// Help command to list all available commands or get detailed information about a specific command.
///
/// The `help` command is a built-in command that provides users with information about the available commands and how to use them. When you invoke the `help` command without any arguments, the bot will display a list of all the commands that it recognizes, along with a brief description of what each command does. This allows users to quickly see what commands are available and get a general idea of their functionality. Additionally, you can specify a specific command as an argument to the `help` command to get more detailed information about that command. This detailed information may include the command's usage syntax, any parameters it may require, and examples of how to use it effectively. The `help` command is an essential tool for users who want to learn more about the bot's capabilities and how to interact with it effectively.
#[poise::command(prefix_command)]
pub async fn help(
    ctx: context::Context<'_>,
    #[description = "Command to ask for help"] command: Option<String>,
) -> Result<()> {
    let config = poise::builtins::HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
