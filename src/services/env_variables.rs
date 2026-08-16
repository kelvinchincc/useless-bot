// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serenity::model::id::GuildId;
use std::env;

use crate::errors::env_variable_error::EnvVariableError;

pub fn token() -> Result<String, EnvVariableError> {
    env::var("TOKEN").map_err(|_| EnvVariableError::RequiredVariableMissing(String::from("TOKEN")))
}

#[allow(dead_code)]
pub fn application_id() -> Result<String, EnvVariableError> {
    env::var("APPLICATION_ID")
        .map_err(|_| EnvVariableError::RequiredVariableMissing(String::from("APPLICATION_ID")))
}

#[allow(dead_code)]
pub fn prefix() -> String {
    env::var("PREFIX")
        .map_err(|e| log::warn!("PREFIX not set, using default '!': {}", e))
        .unwrap_or(String::from("!"))
}

#[allow(dead_code)]
pub fn gulid_id() -> Result<GuildId, EnvVariableError> {
    // let val = env::var("GUILD_ID")
    //     .map_err(|e| log::error!("GUILD_ID is required: {}", e))
    //     .unwrap();
    let val = env::var("GUILD_ID")
        .map_err(|_| EnvVariableError::RequiredVariableMissing(String::from("GUILD_ID")))?;

    let raw = val.parse::<u64>().map_err(|e| {
        EnvVariableError::InvalidVariableFormat(String::from("number"), format!("{:#}", e))
    })?;
    Ok(GuildId::new(raw))
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

#[allow(dead_code)]
pub fn facebook_link_replace_enabled() -> bool {
    let val = env::var("FACEBOOK_LINK_REPLACE_ENABLED").unwrap_or_else(|_| {
        log::debug!("FACEBOOK_LINK_REPLACE_ENABLED not set, default to false");
        "false".to_string()
    });
    val.to_lowercase() == "true"
}

#[allow(dead_code)]
pub fn facebook_alternate_preview() -> bool {
    let val = env::var("FACEBOOK_ALTERNATE_PREVIEW").unwrap_or_else(|_| {
        log::debug!("FACEBOOK_ALTERNATE_PREVIEW not set, default to false");
        "false".to_string()
    });
    val.to_lowercase() == "true"
}
