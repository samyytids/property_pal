use core::panic;

use crate::{condition_functions, utils::{self, handle_unwrapping_some}};
use crate::serializers::{SitemapSet, UrlSet, ConvertUrls};
use reqwest::{Response, Client};
use quick_xml::de::from_str;
use futures;

enum ParseType {
    Sitemap,
    Url
}

async fn make_request(url: &str) -> Option<Response> {
    let result = reqwest::get(url).await;
    if let Ok(value) = result {
        Some(value)
    } else {
        println!("Bad request at: {url}");
        None
    }
}

async fn parse_xml<T: ConvertUrls>(url_set: T, filter_fn: fn(&str) -> bool) -> Vec<String> {
    let urls: Vec<String> = url_set.urls();
    let urls: Vec<String> = urls.into_iter()
        .filter(|url| filter_fn(url))
        .collect();

    urls
}

async fn get_sitemap(url: &str, filter_fn: fn(&str) -> bool) -> Option<Vec<String>> {
    let result = make_request(url).await;
    if let Some(value) = result {
        let xml_data = value.text().await.unwrap_or(String::from(""));
        let url_set: SitemapSet = from_str(&xml_data).unwrap();
        let sitemap_urls = parse_xml(url_set, filter_fn).await;
        Some(sitemap_urls)
    } else {
        None
    }
}

async fn get_multiple_sitemaps(urls: &[String], filter_fn: fn(&str) -> bool, parse_type: ParseType) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let futures: Vec<_> = urls.into_iter()
        .map(|url| get_response(&client, url))
        .collect();

    let responses = futures::future::join_all(futures).await;

    let mut result: Vec<String> = vec![];

    for response in responses {
        match response {
            Ok(content) => {
                let url_set = match parse_type {
                    ParseType::Sitemap => {
                        let result: SitemapSet = from_str(&content).unwrap();
                        let result = parse_xml(result, filter_fn).await;
                        result
                    },
                    ParseType::Url => {
                        let result: UrlSet = from_str(&content).unwrap();
                        let result = parse_xml(result, filter_fn).await;
                        result
                    },
                };
                result.extend(url_set);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    Ok(result)
}

pub async fn get_multiple_image_sitemaps(urls: &[String]) -> Result<Vec<SitemapSet>, reqwest::Error> {
    let client = Client::new();
    let futures: Vec<_> = urls.into_iter()
        .map(|url| get_response(&client, url))
        .collect();

    let responses = futures::future::join_all(futures).await;

    let mut result: Vec<SitemapSet> = vec![];

    for response in responses {
        match response {
            Ok(content) => {
                let image_url_set: SitemapSet = from_str(&content).unwrap();
                result.push(image_url_set)
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    Ok(result)
}

async fn get_response(client: &Client, url: &str) -> Result<String,reqwest::Error> {
    let result = client.get(url).send().await?.text().await?;
    Ok(result)
}

pub async fn async_get(urls: &[String], old: bool) -> Vec<(String, bool)> {
    let client = Client::new();
    let futures: Vec<_> = urls.into_iter()
        .map(|url| get_response(&client, url))
        .collect();

    let responses = futures::future::join_all(futures).await;

    let mut result: Vec<(String, bool)> = vec![];

    for response in responses {
        match response {
            Ok(content) => {
                result.push((content, old));
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    result
}

pub async fn get_urls_to_scrape() -> Vec<String> {
    let sitemap_url = "https://www.propertypal.com/sitemap.xml";

    let base_urls = get_sitemap(&sitemap_url, condition_functions::is_main_sitemap_url).await;
    let base_urls = handle_unwrapping_some(base_urls, &"base sitemap has been changed try again, if it continues to not work send to Sam for refactoring".to_string());
    let base_urls = &base_urls[..];

    let area_urls = get_multiple_sitemaps(base_urls, condition_functions::is_area_sitemap_url, ParseType::Sitemap).await;
    let area_urls = utils::handle_unwrapping_result(area_urls, &"area sitemap has been changed try again, if it continues to not work send to Sam for refactoring".to_string());
    let area_urls = &area_urls[..];

    let area_urls = utils::split_vector(area_urls, 100);
    let mut result: Vec<String> = vec![];
    for chunk in area_urls {
        let property_urls = get_multiple_sitemaps(&chunk, condition_functions::is_area_sitemap_url, ParseType::Url).await.unwrap();
        result.extend(property_urls);
    }

    let result = result.iter().map(|r| {
        if let Some(pos) = r.rfind('/') {
            // Extract the last segment (after the last slash)
            let last_segment = &r[pos + 1..];
    
            // Construct the new URL
            let new_url = format!("https://www.propertypal.com/{}", last_segment);
            return new_url
        } else {
            panic!("Weird url found");
        }
    })
    .collect();

    result
}