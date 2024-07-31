#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, FixedOffset};
use crate::serializer_functions::{string_to_f32, deserialize_difference_percentage};

pub trait ConvertUrls {
    fn urls(&self) -> Vec<String>;
}

#[derive(Debug, Deserialize)]
pub struct SitemapSet {
    #[serde(rename = "sitemap", default)] // This tells Serde to map the sitemap element to the sitemaps field in this struct
    pub urls: Vec<Sitemap>,
}

impl ConvertUrls for SitemapSet {
    fn urls(&self) -> Vec<String> {
        self.urls.iter().map(|entry| entry.loc.clone()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Sitemap {
    pub loc: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlSet {
    #[serde(rename = "url", default)] // This tells Serde to map the sitemap element to the sitemaps field in this struct
    pub urls: Vec<Url>,
}

impl ConvertUrls for UrlSet {
    fn urls(&self) -> Vec<String> {
        self.urls.iter().map(|entry| entry.loc.clone()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub loc: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "loc")] // These are then image:loc, but the image can be dropped
    pub loc: String,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub props: Props
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Props {
    pub page_props: PageProps
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageProps {
    pub property: Property,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Property {
    #[serde(rename = "360Tours")]
    pub tour: Option<Vec<Tour>>,
    pub account: Option<Account>, // if missing means dead property
    pub activation_time: Option<DateTime<FixedOffset>>, // if missing means dead property
    pub address_line_1: String, 
    pub address_line_2: String,
    pub auction: Option<Auction>,
    pub ber: Option<BER>,
    pub ber_exempt: Option<bool>, // if missing means dead property
    pub brief_text: Option<String>,
    pub building_name: String, 
    pub co_ownership_eligible: Option<bool>, // if missing means dead property
    pub coming_soon: Option<bool>, // if missing means dead property
    pub coming_soon_text: Option<String>, 
    pub continuous_relisting: Option<bool>, // if missing means dead property
    pub coordinate: Coordinate,
    pub country: Country,
    pub date_available_from: Option<DateTime<FixedOffset>>,
    pub date_viewable_from: Option<DateTime<FixedOffset>>,
    pub description: Option<String>, // if missing means dead property
    pub display_address: String,
    pub display_address_line_1: String,
    pub display_address_line_2: String,
    pub epc: Option<Epc>,
    pub featurable: Option<bool>, // if missing means dead property
    pub furnished_type: Option<Furnished>,
    pub history: Option<Vec<History>>,
    pub house_number: String, 
    pub id: u32, // Do not use me, seems to not attach to URL is attached to something else.
    pub images: Option<Vec<PropertyImage>>, // if missing means dead property
    pub key_info: Option<Vec<KeyInfo>>, // May need some parsing logic also if missing means dead property
    pub listing_updated_time: Option<DateTime<FixedOffset>>,
    pub num_bathrooms: Option<u32>, 
    pub num_bedrooms: Option<u32>, 
    pub num_reception_rooms: Option<u32>,
    pub ofcom_broadband: Option<Broadband>,
    pub path: String, 
    pub path_id: String,
    pub postcode: String,
    pub price: Option<Price>,
    pub price_tracker_available: Option<bool>, // if missing means dead property
    pub property_type: PropertyType,
    pub published: bool,
    pub region: String, 
    pub sale_type: SaleType,
    pub show_home_opening_time: Option<ShowHomeOpeningTime>, 
    pub site_number: String, 
    pub stats: Option<Stats>,
    pub status: Option<Status>, // if missing means dead property
    pub street: String, 
    pub style: Option<Style>,
    pub style_groups: Option<Vec<StyleGroup>>,
    pub tag_line: Option<String>, // if missing means dead property
    pub time_activated_on_main_website: Option<DateTime<FixedOffset>>, // if missing means dead property
    pub time_last_unpublished_on_p_p: Option<DateTime<FixedOffset>>,
    pub town: String, 
    pub unit_number: String, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tour {
    pub url: String}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Account {
    pub account_number: String,
    pub accreditations: Option<Vec<Accreditation>>,
    pub agent: bool,
    pub developer: bool,
    pub display_address: Option<String>,
    pub enhanced_branding: bool,
    pub include_fee_charges: bool,
    pub missing_accreditations: Option<Vec<Accreditation>>,
    pub office_phone: Option<Phone>,
    pub organisation: String,
    pub organisation_branch: String,
    pub private_developer: bool,
    pub propertypal_awards: Option<Vec<Award>>,
    pub psr_licence_number: Option<String>, 
    pub tier: String,
    pub website_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Accreditation {
    pub id: u32, 
    pub label: String, 
    pub text_key: String, 
    pub tooltip: String,
    #[serde(rename = "type")]
    pub accreditation_type: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Phone {
    pub display_number: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Award {
    pub id: u32, 
    pub label: String, 
    pub text_key: String, 
    pub tooltip: String,
    #[serde(rename = "type")]
    pub award_type: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Auction {
    pub end_time: Option<DateTime<FixedOffset>>,
    pub lot: String,
    pub venue: Option<Venue>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Venue {
    pub address_line_1: String,
    pub address_line_2: String,
    pub id: u32, 
    pub name: String,
    pub online_only: bool,
    pub postcode: String,
    pub region: String, 
    pub town: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct BER {
    pub alphanumeric_rating: Option<String>, 
    pub energy_performance_indicator: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Coordinate {
    pub accuracy: Option<String>, 
    pub latitude: Option<f32>,
    pub longitude: Option<f32>, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Country {
    pub iso_code_2: Option<String>, 
    pub iso_code_3: Option<String>,
    pub name: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Epc {
    pub rating_shorthand: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Furnished {
    pub key: String,
    pub text: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct History {
    pub difference: Option<i32>,
    #[serde(deserialize_with = "deserialize_difference_percentage")]
    pub difference_percentage: Option<f32>,
    pub previous_price: Option<u32>,
    pub previous_published: bool,
    pub previous_status: Option<Status>,
    pub price: u32, 
    pub price_change: bool, 
    pub price_prefix: Option<String>, 
    pub price_suffix: Option<String>, 
    pub published: bool,
    pub published_change: bool,
    pub published_on_awp: bool,
    pub published_on_awp_change: bool,
    pub status: Status,
    pub status_change: bool,
    pub time_modified: Option<DateTime<FixedOffset>>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Status {
    pub key: String, 
    pub text: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]

pub struct PropertyImage {
    pub height: u32, 
    pub width: u32, 
    pub image_type: String, 
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KeyInfo {
    pub group: String, 
    pub key: String, 
    pub name: String,
    pub subtext: Option<String>,
    pub text: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]

pub struct Broadband {
    pub estimated: bool,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_bb_predicted_down: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_bb_predicted_up: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_predicted_down: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_predicted_up: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_sfbb_predicted_down: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_sfbb_predicted_up: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_ufbb_predicted_down: Option<f32>,
    #[serde(deserialize_with = "string_to_f32")]
    pub max_ufbb_predicted_up: Option<f32>,
    pub time_last_cached: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Price {
    pub currency_code: String, 
    pub currency_symbol: String, 
    pub hidden_price_text: String, 
    pub max_price: Option<u32>,
    pub min_price: Option<u32>, 
    pub price: Option<u32>, 
    pub price_hidden: bool, 
    pub price_on_application: bool, 
    pub price_prefix: String, 
    pub price_suffix: String, 
    pub price_type: Option<String>, 
    pub rent_frequency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PropertyType {
    pub key: String, 
    pub text: String, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SaleType {
    pub key: String, 
    pub text: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct ShowHomeOpeningTime {
    pub by_appointment: bool, 
    pub date: Option<DateTime<FixedOffset>>,
    pub days: Vec<u32>,
    pub default_text: String, 
    pub end_hour: Option<u32>, 
    pub text_key: String, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct Stats {
    pub history: Option<Vec<StatHistory>>, //Views
    pub ranking: Option<Ranking>,
    pub total_views: u32
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct StatHistory {
    pub bumped: bool,
    pub date: DateTime<FixedOffset>,
    pub featured: bool,
    pub published: bool,
    pub total_unique_views: u32,
    pub total_views: u32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ranking {
    pub popularity: Option<String>, 
    pub range: Option<Range>,
    pub term: Option<String>, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Range {
    pub from: u32, 
    pub to: u32
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]

pub struct Style {
    pub key: String, 
    pub property_type: String, 
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StyleGroup {
    pub id: u32, 
    pub key: String, 
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Units {

}