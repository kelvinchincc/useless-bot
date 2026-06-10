// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use anyhow::{Context, Result};

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

pub async fn can_safely_previewed(link: &str, curl_user_agent: &str) -> Result<bool> {
    log::info!("Mocking as curl: {}", curl_user_agent);
    log::info!("Checking link: {}", link);

    let client = reqwest::Client::new();
    let res = client
        .get(link)
        .header("User-Agent", curl_user_agent)
        .send()
        .await?;
    let text = res.text().await?;
    log::debug!("Content: {}", text);
    return if text.contains("<meta property=\"og:site_name\" content=\"facebed by pi.kt") {
        Ok(true)
    } else {
        Ok(false)
    };
}

pub async fn get_current_curl_version() -> Result<String> {
    let client = reqwest::Client::new();
    let curl_github_api_url = "https://api.github.com/repos/curl/curl/releases/latest";
    let res = client
        .get(curl_github_api_url)
        .header("User-Agent", "curl-version-checker")
        .send()
        .await?;
    let doc = res.text().await?;
    let parsed = serde_json::from_str::<serde_json::Value>(&doc)?;
    let tag = parsed
        .get("tag_name")
        .context("tag_name not found")?
        .as_str()
        .context("Cannot convert to str")?;
    let version = tag.replace("-", "/").replace("_", ".");

    Ok(version)
}
