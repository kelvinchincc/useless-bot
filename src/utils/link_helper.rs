use std::collections::HashMap;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use anyhow::{Context, Result};
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

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

pub async fn grab_html_og_graph_meta(
    link: &str,
    curl_user_agent: &str,
) -> Result<HashMap<String, String>> {
    let client = reqwest::Client::new();
    let res = client
        .get(link)
        .header("User-Agent", curl_user_agent)
        .send()
        .await?;
    let text = res.text().await?;
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())?;

    let og_data = extract_og_graph(&dom.document);

    Ok(og_data)
}

fn extract_og_graph(handle: &Handle) -> HashMap<String, String> {
    let mut og_data = HashMap::new();
    let node = handle;
    if let NodeData::Element {
        ref name,
        ref attrs,
        ..
    } = node.data
    {
        if name.local.as_ref() == "meta" {
            let mut property = None;
            let mut content = None;
            for attr in attrs.borrow().iter() {
                match attr.name.local.as_ref() {
                    "property" => property = Some(attr.value.to_string()),
                    "content" => content = Some(attr.value.to_string()),
                    _ => {}
                }
            }
            if let (Some(prop), Some(cont)) = (property, content) {
                og_data.insert(prop, cont);
            }
        }
    }
    for child in node.children.borrow().iter() {
        let child_og_data = extract_og_graph(child);
        og_data.extend(child_og_data);
    }
    og_data
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    const CURL_VERSION: &str = "curl/8.21.0";

    #[tokio::test]
    async fn test_should_safely_previewed() {
        let link = "https://facebed.com/share/v/1DJGfShJbS/";
        let result = can_safely_previewed(link, CURL_VERSION).await;
        match result {
            Ok(is_safe) => {
                assert!(is_safe, "Expected the link to be safe for previewing");
            }
            Err(e) => {
                panic!("Error occurred while checking link: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_should_not_safely_previewed() {
        // let link = "https://facebed.com/share/p/1BgEWKif2T/";
        let link = "https://www.facebook.com/share/p/1BgEWKif2T/";
        let result = can_safely_previewed(link, &CURL_VERSION).await;
        match result {
            Ok(is_safe) => {
                assert!(!is_safe, "Expected the link to not be safe for previewing");
            }
            Err(e) => {
                panic!("Error occurred while checking link: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_grab_html_og_graph() {
        let link = "https://facebed.com/share/v/1DJGfShJbS/";
        let result = grab_html_og_graph_meta(link, CURL_VERSION).await;
        let content = result.expect("Failed to grab HTML OG graph meta");
        assert!(
            content.contains_key("og:site_name"),
            "Expected the content to contain the og:site_name meta tag"
        );
        // Should have video
        assert!(
            content.contains_key("og:video"),
            "Expected the content to contain the og:video meta tag"
        );
    }

    #[tokio::test]
    async fn test_grab_html_og_graph_have_photos() {
        let link = "https://www.facebook.com/share/p/1D7Q9gnGmW/";
        let result = grab_html_og_graph_meta(link, CURL_VERSION).await;
        let content = result.expect("Failed to grab HTML OG graph meta");
        assert!(
            content.contains_key("og:title"),
            "Expected the content to contain the og:site_name meta tag"
        );
        // Should have photos
        assert!(
            content.contains_key("og:image"),
            "Expected the content to contain the og:image meta tag"
        );
    }

    #[tokio::test]
    async fn test_og_graph_should_have_multiple_photos() {
        let link = "https://www.facebook.com/share/p/1MQ7wgCPLw/";
        let result = grab_html_og_graph_meta(link, CURL_VERSION).await;
        let content = result.expect("Failed to grab HTML OG graph meta");
        // Should have at least one image
        assert!(
            content.contains_key("og:image"),
            "Expected the content to contain the og:image meta tag"
        );

        // Should have multiple images
        // Check if there are multiple og:image tags
        let image_count = content
            .iter()
            .filter(|(key, _)| key.starts_with("og:image"))
            .count();
        assert!(
            image_count > 1,
            "Expected the content to contain multiple og:image meta tags"
        );
    }
}
