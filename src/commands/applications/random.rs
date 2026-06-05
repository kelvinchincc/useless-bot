// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::{Context, constants::COIN_EMOJI_MAPPING};

/// Random Playground
///
/// A list of playful commands that utilse randomization, such as dice rolls, coin flips, etc.
#[poise::command(slash_command, subcommands("dice", "coin"))]
pub async fn random(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    Ok(())
}

/// Roll a dice with a specified number of faces (default to 6).
#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "faces, default to 6"] faces: Option<u8>,
) -> Result<(), anyhow::Error> {
    let faces = faces.unwrap_or(6);

    if faces < 4 {
        ctx.say("The number of faces must be at least 4.").await?;
        return Ok(());
    }

    let dice_roll = rand::random::<u8>() % faces + 1;
    if faces <= 6 {
        match dice_roll {
            1 => ctx.say(":one:").await?,
            2 => ctx.say(":two:").await?,
            3 => ctx.say(":three:").await?,
            4 => ctx.say(":four:").await?,
            5 => ctx.say(":five:").await?,
            6 => ctx.say(":six:").await?,
            _ => ctx.say(format!("You rolled a {}!", dice_roll)).await?,
        };
    } else {
        ctx.say(format!("You rolled a {}!", dice_roll)).await?;
    }
    Ok(())
}

/// Flip a coin and get either heads or tails.
#[poise::command(slash_command)]
pub async fn coin(ctx: Context<'_>) -> Result<(), anyhow::Error> {
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
