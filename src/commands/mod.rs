// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::data::Data;

pub mod prefixes;

pub fn get_commands() -> Vec<poise::Command<Data, anyhow::Error>> {
    let mut commands = Vec::new();
    commands.extend(prefixes::register_commands());
    commands
}
