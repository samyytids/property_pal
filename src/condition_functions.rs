pub fn is_main_sitemap_url(url: &str) -> bool {
    url.contains("/town/property-for-sale") || url.contains("/town/property-to-rent")
}

pub fn is_area_sitemap_url(_url: &str) -> bool {
    true
}