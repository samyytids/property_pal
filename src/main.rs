#![allow(dead_code)]
mod make_requests;
mod serializers;
mod serializer_functions;
mod condition_functions;
mod utils;
mod sql_functions;

use std::hash::Hash;
use std::io::{Write, Read};
use std::time::Instant;
use std::fs::File;
use std::iter::FromIterator;
use std::collections::HashSet;

use scraper::node::Doctype;
use scraper::{Html, Selector};
use make_requests::async_get;
use reqwest::header::TRANSFER_ENCODING;
use reqwest::Url;
use serde_json::to_string_pretty;

use sqlx::{FromRow, Row};


#[derive(Debug)]
enum StatusOptions {
    NotStc(String),
    Stc(String),
    Unknown(String)
}

impl StatusOptions {
    pub fn new(value: String) -> Self {
        if  value == "toLet" || value == "forSale" || value == "comingSoon" {
            Self::NotStc(value)
        } else if value == "sold" || value == "underOffer" || value == "letAgreed" || value == "saleAgreed" {
            Self::Stc(value)
        } else {
            Self::Unknown(value)
        }
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut transaction  = utils::create_db_managers().await;
    let result = sql_functions::select(&mut transaction).await;
    transaction.commit().await.expect("transaction failed to commit");
    result.iter()
        .for_each(|r| {println!("{:?}", r);println!("")});
    // let urls = make_requests::get_urls_to_scrape().await;

    // let json_data = to_string_pretty(&urls).expect("Failed to process urls to json");
    // let mut file = File::create("./urls.json").expect("Failed to create file");
    // file.write_all(json_data.as_bytes()).expect("Failed to write string to file");
    // let mut file = File::open("urls.json").unwrap();
    // let mut data =  String::new();
    // file.read_to_string(&mut data).unwrap();
    // let urls: Vec<String> = serde_json::from_str(&data).unwrap();

    // // let json = &utils::load_json_data()[..];
    // // utils::delete_and_create_db();
    // let mut transaction  = utils::create_db_managers().await;
    // let urls_to_scrape_local: Vec<String> = sqlx::query(r#"
    //     SELECT t1.url 
    //     FROM property t1
    //     LEFT JOIN dead_property t2 ON t1.id = t2.property_id
    //     WHERE t2.property_id IS NULL
    // "#).fetch_all(&mut *transaction)
    // .await
    // .expect("Failed to query local urls for pre scrape")
    // .iter()
    // .map(|url| url.get("url"))
    // .collect();

    // let urls_to_scrape_local: HashSet<String> = HashSet::from_iter(urls_to_scrape_local);
    // let (urls_to_update, urls_to_add): (Vec<String>, Vec<String>)  = urls.into_iter()
    // .partition(|u| {
    //     urls_to_scrape_local.contains(u)
    // });
    
    // let urls_to_add = utils::split_vector(&urls_to_add, 200);
    // let urls_to_update = utils::split_vector(&urls_to_update, 200);

    // let mut result: Vec<(String, bool)> = vec![];
    // for urls in urls_to_add {
    //     result.extend(async_get(&urls[..], false).await);
    // }

    // for urls in urls_to_update {
    //     result.extend(async_get(&urls[..], true).await);
    // }

    // let result: Vec<(Result<serializers::Data, serde_json::Error>, bool)> = result.iter()
    //     .map(|r| {
    //         let html = Html::parse_document(&r.0);
    //         let selector = Selector::parse("script#__NEXT_DATA__").expect("Missing script tag");
    //         let script = html.select(&selector).next().map(|element| {element.inner_html()}).expect("Failed to extract internal HTML");
    //         let parsed: Result<serializers::Data, serde_json::Error> = serde_json::from_str(&script);
    //         (parsed, r.1)
    //     })
    //     .collect();
    
    // for element in result {
    //     let data = element.0;
    //     let old = element.1;
    //     let property = match data {
    //         Ok(d) => d.props.page_props.property,
    //         Err(_) => continue
    //     };
    //     let account = &property.account;
    //     let status = match &property.status {
    //         Some(s) => Some(StatusOptions::new(s.key.clone())),
    //         None => None
    //     };

    //     match (account, status, old) {
    //         (Some(a), Some(StatusOptions::NotStc(_)), false) => { 
    //             sql_functions::healthy_inserts(&property, &a, &mut *transaction).await;
    //         },
    //         (Some(a), Some(StatusOptions::Stc(s)), false) => { 
    //             sql_functions::healthy_inserts(&property, &a, &mut *transaction).await;
    //             sql_functions::dead_inserts(&property, &mut transaction, true, false, Some(s)).await;
    //         },
    //         (Some(_), Some(StatusOptions::NotStc(_)), true) => { 
    //             sql_functions::updates(&property, &mut transaction).await;
    //         },
    //         (Some(_), Some(StatusOptions::Stc(s)), true) => { 
    //             sql_functions::updates(&property, &mut transaction).await;
    //             sql_functions::dead_inserts(&property, &mut transaction, true, false, Some(s)).await;
    //         },
    //         (None, None, _) => {
    //             sql_functions::dead_inserts(&property, &mut transaction, false, true, None).await;
                
    //         },
    //         (_, _, _) => println!("Property doesn't fit the above categories: {}", property.path_id),
    //     };
    // }

    // let alive = sqlx::query("SELECT COUNT(*) AS count FROM property").fetch_one(&mut *transaction).await.unwrap();
    // let dead = sqlx::query("SELECT COUNT(*) AS count FROM dead_property").fetch_one(&mut *transaction).await.unwrap();
    // let alive: i64 = alive.get("count");
    // let dead: i64 = dead.get("count");

    // println!("{} {}", alive, dead);
    // transaction.commit().await.expect("transaction failed to commit");

    let duration = start.elapsed();
    println!("{:?}", duration);
}
