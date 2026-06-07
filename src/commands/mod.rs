// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::types::data::Data;

pub mod applications;
pub mod prefixes;

pub fn get_commands() -> Vec<poise::Command<Data, anyhow::Error>> {
    let mut commands = Vec::new();
    commands.extend(applications::register_commands());
    commands.extend(prefixes::register_commands());
    commands
}
