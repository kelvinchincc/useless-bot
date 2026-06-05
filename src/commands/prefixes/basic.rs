// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::services::env_variables;
use anyhow::Error;
use poise::CreateReply;
use std::default::Default;

use crate::Context;

fn get_hi_help_text() -> String {
    let msg = concat!(
        "Introduce the bot and what it is about, with a banner and link to the github repository."
    );
    String::from(msg)
}

#[poise::command(prefix_command, help_text_fn = "get_hi_help_text")]
pub async fn hi(ctx: Context<'_>) -> Result<(), Error> {
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
//         .description(format!("{} is a bot based on [useless-bot](https://github.com/kelvinchin12070811/useless-bot).
// This bot have no idea what it can do nor what it will do, it is just created for exporing how a discord
// bot can do or what it can achive.", name.unwrap_or_else(|| "Useless Bot".to_string())))
        .description(format!(concat!(
            "{} is a bot based on [useless-bot](https://github.com/kelvinchin12070811/useless-bot). ",
            "This bot have no idea what it can do nor what it will do, it is just created for exporing how a discord ",
            "bot can do or what it can achive.\n\nNot sure where to start? Try try `!help`!"
        ), name.unwrap_or("Useless Bot".to_string())))
        .image("https://raw.githubusercontent.com/kelvinchin12070811/useless-bot/refs/heads/dev/banner.jpg")
        .url("https://github.com/kelvinchin12070811/useless-bot")
        // 0x010409
        .color((1, 4, 9));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}

fn get_ping_help_text() -> String {
    let msg = concat!(
        "A simple command to test if the bot is responsive. When you use the `ping` command, the bot will",
        "reply with `Pong!`, indicating that it is online and able to respond to commands. This is a common command",
        "used in many bots to check their responsiveness and connectivity.",
    );
    String::from(msg)
}

#[poise::command(prefix_command, help_text_fn = "get_ping_help_text")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(":ping_pong: Pong!").await?;
    Ok(())
}

fn get_help_help_text() -> String {
    String::from(concat!(
        "The `help` command provides information about the available commands and how to use them. When you use the `help` command,",
        "the bot will display a list of all the commands that it recognizes, along with a brief description of what each command does.",
        "You can also specify a specific command as an argument to get more detailed information about that command, including its usage and any parameters it may require."
    ))
}

#[poise::command(prefix_command, help_text_fn = "get_help_help_text")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to ask for help"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
