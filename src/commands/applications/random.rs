// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Context;

#[poise::command(slash_command, subcommands("dice"))]
pub async fn random(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    Ok(())
}

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
