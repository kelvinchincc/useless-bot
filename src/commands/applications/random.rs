// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Result;
use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::{constants::COIN_EMOJI_MAPPING, types::context};

/// Random Playground
///
/// A list of playful commands that utilse randomization, such as dice rolls, coin flips, etc.
#[poise::command(slash_command, subcommands("dice", "coin", "choose"))]
pub async fn random(_ctx: context::Context<'_>) -> Result<()> {
    Ok(())
}

/// Roll a dice with a specified number of faces (default to 6).
#[poise::command(slash_command)]
pub async fn dice(
    ctx: context::Context<'_>,
    #[description = "faces, default to 6"] faces: Option<u8>,
) -> Result<()> {
    let faces = faces.unwrap_or(6);

    if faces < 4 {
        ctx.say("The number of faces must be at least 4.").await?;
        return Ok(());
    }

    let dice_roll = rand::random::<u8>() % faces + 1;
    if faces <= 6 {
        let msg = match dice_roll {
            1 => String::from(":one:"),
            2 => String::from(":two:"),
            3 => String::from(":three:"),
            4 => String::from(":four:"),
            5 => String::from(":five:"),
            6 => String::from(":six:"),
            _ => format!("You rolled a {}!", dice_roll),
        };
        ctx.say(msg).await?;
    } else {
        ctx.say(format!("You rolled a {}!", dice_roll)).await?;
    }
    Ok(())
}

/// Flip a coin and get either heads or tails.
#[poise::command(slash_command)]
pub async fn coin(ctx: context::Context<'_>) -> Result<()> {
    let flip = rand::random::<bool>();
    let side = COIN_EMOJI_MAPPING[flip as usize];
    let title = if flip { "Tails" } else { "Heads" };
    let embed = CreateEmbed::default().title(title).thumbnail(side).footer(
        CreateEmbedFooter::new("Icon made by emoji.gg")
            .icon_url("https://emoji.gg/assets/img/logo.png?v=2"),
    );
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;
    Ok(())
}

/// Choose randomly between multiple options provided by the user.
///
/// This command allows users to input multiple options, and the bot will randomly select one of them. It is useful for making decisions or adding an element of chance to a conversation.
#[poise::command(slash_command)]
pub async fn choose(
    ctx: context::Context<'_>,
    #[description = "Options to choose from"] options: String,
    #[description = "Saperator for options, default to comma"] separator: Option<String>,
) -> Result<()> {
    let saperator = separator.unwrap_or(",".to_string());
    let options: Vec<&str> = options.split(&saperator).map(|s| s.trim()).collect();
    if options.len() < 2 {
        ctx.say("Please provide at least two options separated by commas.")
            .await?;
        return Ok(());
    }
    let choice = options[(rand::random::<u32>() % options.len() as u32) as usize];
    ctx.say(format!("I choose: {}", choice)).await?;
    Ok(())
}
