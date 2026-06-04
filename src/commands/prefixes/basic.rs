// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Error;
use poise::CreateReply;

use crate::Context;

#[poise::command(prefix_command)]
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
        .title("Oh Hi!")
        .description(format!("{} is a bot based on [useless-bot](https://github.com/kelvinchin12070811/useless-bot).
This bot have no idea what it can do nor what it will do, it is just created for exporing how a discord
bot can do or what it can achive.", name.unwrap_or_else(|| "Useless Bot".to_string())))
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

#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Ping and Pong!"] _command: Option<String>,
) -> Result<(), Error> {
    ctx.say(":ping_pong: Pong!").await?;
    Ok(())
}
