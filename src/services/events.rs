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
    if let Consumed(true) = facebook_link_replace_filter(new_message, ctx, data).await? {
        return Ok(());
    }
    if let Consumed(true) = keyword_response_filter(new_message, ctx, data).await? {
        return Ok(());
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
    match template {
        Some(response) => {
            let response_text = match response {
                crate::data::KeywordResponse::Value(s) => s.clone(),
                crate::data::KeywordResponse::List(arr) => {
                    let idx = rand::random::<u32>() % (arr.len() as u32);
                    arr[idx as usize].clone()
                }
            };

            if let Err(e) = message.channel_id.say(&ctx.http, response_text).await {
                log::error!("Failed to send message: {}", e);
                return Ok(Consumed(false));
            }
        }
        None => {
            return Ok(Consumed(false));
        }
    }

    Ok(Consumed(true))
}

/// This filter replace the facebook link with facebed so it can be previewed in discord.
///
/// This is a workaround for the fact that facebook link cannot be previewed in discord. By replacing the link with facebed, it can be previewed in discord.
///
/// Eg: https://www.facebook.com/username/posts/1234567890 will be replaced with https://www.facebed.com/username/posts/1234567890. This is a temporary solution until facebook fix the issue.
async fn facebook_link_replace_filter(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
    _data: &Data,
) -> Result<Consumed, anyhow::Error> {
    log::debug!("Link: {:?}", message.content);

    // Grab the first facebook.com link in the message, if any which starts with www.facebook.com or m.facebook.com
    let facebook_link_regex = regex::Regex::new(r"(https?://)?(www|m)\.facebook\.com/[^\s]+");
    if let Err(e) = facebook_link_regex {
        log::error!("Failed to create regex: {}", e);
        return Ok(Consumed(false));
    }
    let first_facebook_link = facebook_link_regex.unwrap().find(&message.content);

    if let Some(link) = first_facebook_link {
        let original_link = link.as_str();
        let replaced_link = if original_link.starts_with("https://www.") {
            original_link.replace("www.facebook.com", "facebed.com")
        } else if original_link.starts_with("https://m.") {
            original_link.replace("m.facebook.com", "facebed.com")
        } else {
            original_link.to_string()
        };

        if let Err(e) = message
            .channel_id
            .say(&ctx.http, format!("{}", replaced_link))
            .await
        {
            log::error!("Failed to send message: {}", e);
            return Ok(Consumed(false));
        }
    }

    Ok(Consumed(true))
}
