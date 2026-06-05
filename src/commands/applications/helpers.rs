// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use poise::{ChoiceParameter, command};

use crate::Context;

#[derive(Debug, ChoiceParameter)]
enum LmgtfySearchEngine {
    #[name = "Google"]
    Google,
    #[name = "Bing"]
    Bing,
    #[name = "DuckDuckGo"]
    DuckDuckGo,
}

/// Helpers
///
/// Accessory commands that support daily operations, such as let me google that for you, etc
#[command(slash_command, subcommands("lmgtfy"))]
pub async fn helpers(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    Ok(())
}

/// let me google that for you
#[command(slash_command)]
pub async fn lmgtfy(
    ctx: Context<'_>,
    #[description = "The query you want to google"] query: String,
    #[description = "The search engine to use, default to google"] search_engine: Option<
        LmgtfySearchEngine,
    >,
) -> Result<(), anyhow::Error> {
    let address = match search_engine.unwrap_or(LmgtfySearchEngine::Google) {
        LmgtfySearchEngine::Google => "https://www.google.com/search?q=",
        LmgtfySearchEngine::Bing => "https://www.bing.com/search?q=",
        LmgtfySearchEngine::DuckDuckGo => "https://duckduckgo.com/?q=",
    };
    ctx.say(format!("{}{}", address, urlencoding::encode(&query)))
        .await?;
    Ok(())
}
