// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub fn token() -> String {
    std::env::var("TOKEN").expect("DISCORD_BOT_TOKEN must be set")
}

#[allow(dead_code)]
pub fn application_id() -> String {
    std::env::var("APPLICATION_ID").expect("APPLICATION_ID must be set")
}

#[allow(dead_code)]
pub fn prefix() -> String {
    let prefix = std::env::var("PREFIX");
    match prefix {
        Ok(p) => p,
        Err(_) => {
            log::warn!("PREFIX not set, using default '!' prefix");
            "!".to_string()
        }
    }
}

#[allow(dead_code)]
pub fn gulid_id() -> serenity::model::id::GuildId {
    let val = std::env::var("GUILD_ID").expect("GUILD_ID must be set");
    let raw = val.parse::<u64>().expect("GUILD_ID must be all number");
    serenity::model::id::GuildId::new(raw)
}
