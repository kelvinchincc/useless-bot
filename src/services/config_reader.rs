// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::types::data::KeywordResponse;

pub fn parse_keyword_response_config() -> HashMap<String, KeywordResponse> {
    let cwd = std::env::current_dir()
        .map_err(|_| log::error!("Failed to get current working directory"))
        .unwrap();
    let config_path = cwd.join("data").join("keyword_responses.yaml");

    log::info!("Reading keyword response from: {}", config_path.display());

    let config_str = std::fs::read_to_string(config_path).unwrap_or("".to_string());
    let config: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&config_str)
        .map_err(|_| log::error!("Failed to parse keyword_responses.yaml"))
        .unwrap();
    log::debug!("Parsed YAML config: {:?}", config);

    let mut keyword_response_dict: HashMap<String, KeywordResponse> = HashMap::new();

    for (key, value) in config {
        if let Some(s) = value.as_str() {
            log::debug!("Parsing string for key '{}': {}", key, s);
            keyword_response_dict.insert(key, KeywordResponse::Value(s.to_string()));
        } else if let Some(arr) = value.as_sequence() {
            log::debug!("Parsing list for key '{}': {:?}", key, arr);
            let responses = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            log::debug!("Parsed list for key '{}': {:?}", key, responses);
            keyword_response_dict.insert(key, KeywordResponse::List(responses));
        }
    }

    log::debug!(
        "Loaded keyword response dict: {:?}",
        keyword_response_dict.keys().collect::<Vec<&String>>()
    );

    return keyword_response_dict;
}
