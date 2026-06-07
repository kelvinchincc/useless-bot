// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

const URL_REGEX: &str = r"^https://(www|m)\.facebook\.com/\S*$";

pub fn get_pure_facebook_link(link: &str) -> Option<String> {
    let link = link.trim();
    let re = regex::Regex::new(URL_REGEX).unwrap();

    if re.is_match(link) {
        Some(String::from(link.replace(
            "https://m.facebook.com",
            "https://www.facebook.com",
        )))
    } else {
        None
    }
}
