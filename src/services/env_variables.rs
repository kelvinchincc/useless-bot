// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serenity::model::id::GuildId;
use std::env;

pub fn token() -> String {
    env::var("TOKEN").expect("DISCORD_BOT_TOKEN must be set")
}

#[allow(dead_code)]
pub fn application_id() -> String {
    env::var("APPLICATION_ID").expect("APPLICATION_ID must be set")
}

#[allow(dead_code)]
pub fn prefix() -> String {
    let prefix = env::var("PREFIX");
    match prefix {
        Ok(p) => p,
        Err(_) => {
            log::warn!("PREFIX not set, using default '!' prefix");
            "!".to_string()
        }
    }
}

#[allow(dead_code)]
pub fn gulid_id() -> GuildId {
    let val = env::var("GUILD_ID").expect("GUILD_ID must be set");
    let raw = val.parse::<u64>().expect("GUILD_ID must be all number");
    GuildId::new(raw)
}

#[allow(dead_code)]
pub fn greeting() -> String {
    env::var("GREETING").unwrap_or_else(|_| {
        log::warn!("GREETING not set, using default greeting");
        "Oh Hi!".to_string()
    })
}

#[allow(dead_code)]
pub fn use_guild_commands() -> bool {
    let val = env::var("USE_GUILD_COMMANDS").unwrap_or_else(|_| {
        log::info!("USE_GUILD_COMMANDS not set, default to false");
        "false".to_string()
    });
    val.to_lowercase() == "true"
}
