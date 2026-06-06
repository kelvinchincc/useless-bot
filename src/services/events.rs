// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use serenity::client::FullEvent;

use crate::data::Data;

pub async fn handle_events(
    ctx: &serenity::prelude::Context,
    event: &FullEvent,
    _framework: &poise::FrameworkContext<'_, Data, anyhow::Error>,
    data: &Data,
) -> Result<(), anyhow::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            log::info!("Bot is ready! Username: {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            if let Err(e) = dispatch_text_process_filters(new_message, ctx, data).await {
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
    data: &Data,
) -> Result<(), anyhow::Error> {
    let filters = vec![keyword_response_filter(new_message, ctx, data)];
    for filter in filters {
        let consumed = filter.await?;
        if consumed.0 {
            break;
        }
    }

    Ok(())
}

async fn keyword_response_filter(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
    data: &Data,
) -> Result<Consumed, anyhow::Error> {
    let text = message.content.trim();
    let text_len_utf8 = text.chars().count();
    log::debug!("Text: {:?}", text);
    log::debug!("Message length: {}", text_len_utf8);

    // if the message is longer than 15 characters, we ignore it
    if text_len_utf8 > 20 {
        return Ok(Consumed(false));
    }

    let template = data.keyword_response_dict.get(text);
    if let Some(response) = template {
        let response_text = match response {
            crate::data::KeywordResponse::Value(s) => s.clone(),
            crate::data::KeywordResponse::List(arr) => {
                let idx = rand::random::<u32>() % (arr.len() as u32);
                arr[idx as usize].clone()
            }
        };

        if let Err(e) = message.channel_id.say(&ctx.http, response_text).await {
            log::error!("Failed to send message: {}", e);
        }
    }

    Ok(Consumed(true))
}
