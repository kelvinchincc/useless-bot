use poise::CreateReply;
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use serenity::client::FullEvent;

use crate::{
    services::env_variables,
    types::data::{Data, KeywordResponse},
    utils::link_helper::{self, can_safely_previewed, get_pure_facebook_link},
};
use anyhow::{Error, Result};

const AUTO_DELETE_INTERVAL_SECONDS: u64 = 10;

pub async fn handle_events(
    ctx: &serenity::prelude::Context,
    event: &FullEvent,
    _framework: &poise::FrameworkContext<'_, Data, anyhow::Error>,
    data: &Data,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            let mut data = data.user_name.write().await;
            log::info!("Bot is ready! Username: {}", data_about_bot.user.name);
            *data = data_about_bot.user.name.clone();
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

enum FilterResult {
    Consumed,
    NotConsumed,
}

fn result_helper(func: Result<FilterResult, Error>) -> bool {
    let result = func
        .map_err(|e| log::error!("Error in filter: {:#}", e))
        .unwrap_or(FilterResult::NotConsumed);
    match result {
        FilterResult::Consumed => true,
        FilterResult::NotConsumed => false,
    }
}

async fn dispatch_text_process_filters(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
    data: &Data,
) -> Result<()> {
    if result_helper(facebook_link_replace_filter(message, ctx, data).await) {
        return Ok(());
    }
    if result_helper(keyword_response_filter(message, ctx, data).await) {
        return Ok(());
    }

    Ok(())
}

async fn keyword_response_filter(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
    data: &Data,
) -> Result<FilterResult> {
    let text = message.content.trim();
    let text_len_utf8 = text.chars().count();
    log::debug!("Text: {:?}", text);
    log::debug!("Message length: {}", text_len_utf8);

    // if the message is longer than 15 characters, we ignore it
    if text_len_utf8 > 15 {
        return Ok(FilterResult::NotConsumed);
    }

    let template = data.keyword_response_dict.get(text);
    let Some(response) = template else {
        log::debug!("No keyword response found for: {}", text);
        return Ok(FilterResult::NotConsumed);
    };

    let response_text = match response {
        KeywordResponse::Value(s) => s.clone(),
        KeywordResponse::List(arr) => {
            let idx = rand::random::<u32>() % (arr.len() as u32);
            arr[idx as usize].clone()
        }
    };

    if let Err(e) = message.reply(ctx, response_text).await {
        log::error!("Failed to send message: {:#}", e);
        return Ok(FilterResult::NotConsumed);
    }

    Ok(FilterResult::Consumed)
}

/// This filter replace the facebook link with facebed so it can be previewed in discord.
///
/// This is a workaround for the fact that facebook link cannot be previewed in discord. By replacing the link with facebed, it can be previewed in discord.
///
/// Eg: https://www.facebook.com/username/posts/1234567890 will be replaced with https://www.facebed.com/username/posts/1234567890. This is a temporary solution until facebook fix the issue.
async fn facebook_link_replace_filter(
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
    data: &Data,
) -> Result<FilterResult, anyhow::Error> {
    if !crate::services::env_variables::facebook_link_replace_enabled() {
        return Ok(FilterResult::NotConsumed);
    }

    log::debug!("Link: {:?}", message.content);

    let parts = message
        .content
        .split("\n")
        .filter(|line| line.trim().len() > 0)
        .collect::<Vec<&str>>();

    let first = parts.first().unwrap_or(&"").trim();
    let last = parts.last().unwrap_or(&"").trim();
    let link_to_process = get_pure_facebook_link(first)
        .or_else(|| get_pure_facebook_link(last))
        .unwrap_or_default();

    if link_to_process.len() <= 0 {
        log::debug!("No facebook link found in message: {}", message.content);
        return Ok(FilterResult::NotConsumed);
    }

    let replaced_link = link_to_process.replace("www.facebook.com", "facebed.com");
    let text = link_helper::get_content(&replaced_link, &data.curl_user_agent).await?;
    let can_preview = can_safely_previewed(text.as_str())?;
    if !can_preview {
        log::error!("Link cannot preview by facebed: {}", link_to_process);
        let msg = message
            .reply(
                ctx,
                format!(
                    concat!(
                        "Sorry, this link cannot be previewed due to Facebook's restrictions.\n\n",
                        "This message will be deleted in {} seconds."
                    ),
                    AUTO_DELETE_INTERVAL_SECONDS
                ),
            )
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(AUTO_DELETE_INTERVAL_SECONDS)).await;
        msg.delete(ctx).await?;

        return Ok(FilterResult::Consumed);
    }

    if env_variables::facebook_alternate_preview() {
        alternate_facebook_link_preview(text.as_str(), &replaced_link, data, message, ctx).await?;
    } else {
        message.reply(ctx, replaced_link).await?;
    }
    Ok(FilterResult::Consumed)
}

// Helper functions
async fn alternate_facebook_link_preview(
    text: &str,
    replaced_link: &str,
    _data: &Data,
    message: &serenity::model::channel::Message,
    ctx: &serenity::prelude::Context,
) -> Result<()> {
    use serenity::builder::*;

    let content = link_helper::grab_html_og_graph_meta(text)?;
    let site_name = content
        .get("og:site_name")
        .and_then(|c| c.first())
        .map(|c| c.clone())
        .unwrap_or("Facebook".to_string());
    let title = content
        .get("og:title")
        .and_then(|c| c.first())
        .map(|c| c.clone())
        .unwrap_or_default();
    let description = content
        .get("og:description")
        .and_then(|c| c.first())
        .map(|c| c.clone())
        .unwrap_or_default();
    let images = match content.get("og:image") {
        // Take only first 4 images.
        Some(img) => img
            .iter()
            .take(4)
            .map(|s| s.clone())
            .collect::<Vec<String>>(),
        None => vec![],
    };

    let embeds = match images.len() {
        0 => vec![
            CreateEmbed::default()
                .description(&description)
                .title(&title)
                .footer(CreateEmbedFooter::new(&site_name))
                .url(replaced_link),
        ],
        _ => images
            .iter()
            .enumerate()
            .map(|(index, img)| {
                let mut embed = CreateEmbed::default().image(img).url(replaced_link);

                if index == 0 {
                    embed = embed
                        .description(&description)
                        .title(&title)
                        .footer(CreateEmbedFooter::new(&site_name));
                }

                embed
            })
            .collect::<Vec<CreateEmbed>>(),
    };

    let reply = CreateMessage::default()
        .embeds(embeds)
        .reference_message(message);
    let video = content.get("og:video").and_then(|c| c.first());

    message.channel_id.send_message(&ctx.http, reply).await?;
    if let Some(video) = video {
        let builder = CreateMessage::default().content(format!("[Video]({})", video));
        message.channel_id.send_message(&ctx.http, builder).await?;
    }

    Ok(())
}
