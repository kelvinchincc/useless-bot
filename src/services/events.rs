// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use serenity::client::FullEvent;

use crate::data::Data;

pub async fn handle_events(
    ctx: &serenity::prelude::Context,
    event: &FullEvent,
    _framework: &poise::FrameworkContext<'_, Data, anyhow::Error>,
    _data: &Data,
) -> Result<(), anyhow::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            log::info!("Bot is ready! Username: {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            if let Err(e) = dispatch_text_process_filters(new_message, ctx).await {
                log::error!("Failed to process message: {}", e);
            }
        }
        _ => {}
    }

    Ok(())
}

struct Consumed(bool);

async fn dispatch_text_process_filters(
    new_message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
) -> Result<(), anyhow::Error> {
    let filters = vec![keywork_response_filter(new_message, ctx)];
    for filter in filters {
        let consumed = filter.await?;
        if consumed.0 {
            break;
        }
    }

    Ok(())
}

async fn keywork_response_filter(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
) -> Result<Consumed, anyhow::Error> {
    let text = message.content.trim();
    log::debug!("Text: {:?}", text);
    // if the message is not exactly one work, ignore it
    if text.split_whitespace().count() != 1 {
        return Ok(Consumed(false));
    }

    match text {
        "rich" => {
            if let Err(e) = message.reply(ctx, "Ya lor!").await {
                log::error!("Failed to reply to message: {}", e);
            }
        }
        _ => {}
    };

    Ok(Consumed(true))
}
