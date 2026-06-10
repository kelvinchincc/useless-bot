// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use std::collections::HashMap;

pub struct Data {
    pub keyword_response_dict: HashMap<String, KeywordResponse>,
    pub curl_user_agent: String,
}

pub enum KeywordResponse {
    Value(String),
    List(Vec<String>),
}
