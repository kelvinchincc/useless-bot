use std::collections::HashMap;

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use anyhow::{Context, Result};
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

const URL_REGEX: &str = r"^https://(www|m)\.facebook\.com/\S*$";

pub async fn get_content(link: &str, curl_user_agent: &str) -> Result<String> {
    log::info!("Mocking as curl: {}", curl_user_agent);
    log::info!("Fetching content from: {}", link);

    let client = reqwest::Client::new();
    let res = client
        .get(link)
        .header("User-Agent", curl_user_agent)
        .send()
        .await?;
    let text = res.text().await?;

    Ok(text)
}

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

pub fn can_safely_previewed(text: &str) -> Result<bool> {
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

pub fn grab_html_og_graph_meta(text: &str) -> Result<HashMap<String, Vec<String>>> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())?;

    let og_data = extract_og_graph(&dom.document);

    Ok(og_data)
}

fn extract_og_graph(handle: &Handle) -> HashMap<String, Vec<String>> {
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
                og_data.entry(prop).or_insert_with(Vec::new).push(cont);
            }
        }
    }
    for child in node.children.borrow().iter() {
        let child_og_data = extract_og_graph(child);
        for (key, mut values) in child_og_data {
            og_data
                .entry(key)
                .or_insert_with(Vec::new)
                .append(&mut values);
        }
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
        let text = get_content(link, CURL_VERSION)
            .await
            .expect("Failed to fetch content");
        let result = can_safely_previewed(text.as_str());
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
        let text = get_content(link, CURL_VERSION)
            .await
            .expect("Failed to fetch content");
        let result = can_safely_previewed(text.as_str());
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
        let text = get_content(link, CURL_VERSION)
            .await
            .expect("Failed to fetch content");
        let result = grab_html_og_graph_meta(text.as_str());
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
        let text = get_content(link, CURL_VERSION)
            .await
            .expect("Failed to fetch content");
        let result = grab_html_og_graph_meta(text.as_str());
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
        let link = "https://facebed.com/share/p/1MQ7wgCPLw/";
        let text = get_content(link, CURL_VERSION)
            .await
            .expect("Failed to fetch content");
        let result = grab_html_og_graph_meta(text.as_str());
        let content = result.expect("Failed to grab HTML OG graph meta");
        // Should have at least one image
        assert!(
            content.contains_key("og:image"),
            "Expected the content to contain the og:image meta tag"
        );

        // Should have multiple images
        // Check if there are multiple og:image tags
        println!("Content: {:?}", content.get("og:image"));
        let image_count = content.get("og:image").map(|c| c.len()).unwrap_or_default();
        assert!(
            image_count > 1,
            "Expected the content to contain multiple og:image meta tags"
        );
    }
}
