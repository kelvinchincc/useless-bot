// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anyhow::Result;
use poise::{ChoiceParameter, command};

use crate::types::context;

#[derive(Debug, ChoiceParameter)]
enum LmgtfySearchEngine {
    #[name = "Google"]
    Google,
    #[name = "Bing"]
    Bing,
    #[name = "DuckDuckGo"]
    DuckDuckGo,
    #[name = "Perplexity"]
    Perplexity,
}

/// Helpers
///
/// Accessory commands that support daily operations, such as let me google that for you, etc
#[command(slash_command, subcommands("lmgtfy"))]
pub async fn helpers(_ctx: context::Context<'_>) -> Result<()> {
    Ok(())
}

/// let me google that for you
#[command(slash_command)]
pub async fn lmgtfy(
    ctx: context::Context<'_>,
    #[description = "The query you want to google"] query: String,
    #[description = "The search engine to use, default to google"] search_engine: Option<
        LmgtfySearchEngine,
    >,
) -> Result<()> {
    let search_engine = search_engine.unwrap_or(LmgtfySearchEngine::Google);
    let address = match search_engine {
        LmgtfySearchEngine::Google => "https://www.google.com/search?q=",
        LmgtfySearchEngine::Bing => "https://www.bing.com/search?q=",
        LmgtfySearchEngine::DuckDuckGo => "https://duckduckgo.com/search?q=",
        LmgtfySearchEngine::Perplexity => "https://www.perplexity.ai/search?q=",
    };
    ctx.say(format!("{}{}", address, urlencoding::encode(&query)))
        .await?;
    Ok(())
}
