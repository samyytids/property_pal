use crate::serializers::{self, *};
use chrono::FixedOffset;
use sqlx::{Error, SqliteConnection};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::Row;
use crate::utils::{handle_db_errors, convert_date_to_string};

async fn insert_account(account: &Account, transaction: &mut SqliteConnection) {
    let phone_number = match &account.office_phone {
        Some(phone) => Some(phone.display_number.clone()),
        None => None
    };
    let result = sqlx::query(r#"
        INSERT INTO account 
        (
            id, agent, developer, enchanced_branding, include_fee_charges,
            phone, organisation, organisation_branch, private_developer,
            psr_license_number, tier, website_url, display_address
        ) VALUES 
        (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
        )
    "#)
        .bind(&account.account_number)
        .bind(&account.agent)
        .bind(&account.developer)
        .bind(&account.enhanced_branding)
        .bind(&account.include_fee_charges)
        .bind(&phone_number)
        .bind(&account.organisation)
        .bind(&account.organisation_branch)
        .bind(&account.private_developer)
        .bind(&account.psr_licence_number)
        .bind(&account.tier)
        .bind(&account.website_url)
        .bind(&account.display_address)
        .execute(&mut *transaction)
        .await;
    handle_db_errors::<SqliteQueryResult, Error>(result, &account.account_number, "Account");
    


}

async fn insert_accreditation(accreditations: &Option<Vec<Accreditation>>, account_id: &str, transaction: &mut SqliteConnection, have: bool) {
    if let Some(accreditations) = &accreditations {
        for a in accreditations {
            let result = sqlx::query(r#"
                INSERT OR IGNORE INTO accreditation 
                (
                    id, label, text_key, tooltip, accreditation_type, url
                ) VALUES 
                (
                    $1, $2, $3, $4, $5, $6
                )
            "#)
            .bind(&a.id)
            .bind(&a.label)
            .bind(&a.text_key)
            .bind(&a.tooltip)
            .bind(&a.accreditation_type)
            .bind(&a.url)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &a.id.to_string(), "Accreditations");

            let result = sqlx::query(r#"
                INSERT INTO accreditation_account 
                (
                    account_id, accreditation_id, have
                ) VALUES 
                (
                    $1, $2, $3
                )
            "#)
            .bind(&account_id)
            .bind(&a.id)
            .bind(have)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &account_id, "Accreditations_account");
        }
    }
}

async fn insert_award(account: &Account, transaction: &mut SqliteConnection) {
    let account_id = &account.account_number;
    if let Some(awards) = &account.propertypal_awards {
        for a in awards {
            // Execute the query within the async context
            let result = sqlx::query(r#"
                INSERT INTO award 
                (
                    id, label, text_key, tooltip, award_type, url
                ) VALUES 
                (
                    $1, $2, $3, $4, $5, $6
                )
            "#)
            .bind(&a.id)
            .bind(&a.label)
            .bind(&a.text_key)
            .bind(&a.tooltip)
            .bind(&a.award_type)
            .bind(&a.url)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &account_id, "award");

            let result = sqlx::query(r#"
                INSERT INTO award_account 
                (
                    account_id, award_id, have
                ) VALUES 
                (
                    $1, $2, $3
                )
            "#)
            .bind(account_id)
            .bind(&a.id)
            .bind(true)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &account_id, "award_account");
        }
    }
}

async fn insert_tour(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(tours) = &property.tour { 
        for tour in tours {
            let result = sqlx::query(r#"
                INSERT INTO tour 
                (
                    property_id, url
                ) VALUES 
                (
                    $1, $2
                )
            "#)
            .bind(property_id)
            .bind(&tour.url)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "tour");
        }
    }
}

async fn insert_property(property: &Property, transaction: &mut SqliteConnection) {
    let date_activated = convert_date_to_string(&property.activation_time);
    let date_available = convert_date_to_string(&property.date_available_from);
    let date_viewable = convert_date_to_string(&property.date_viewable_from);
    let date_activated_on_site = convert_date_to_string(&property.time_activated_on_main_website);
    let date_unpublished = convert_date_to_string(&property.time_last_unpublished_on_p_p);

    let status = property.status.as_ref();
    let status = match status {
        Some(s) => Some(s.key.clone()),
        None => None
    };
    let url = format!("https://www.propertypal.com/{}",&property.path_id);

    let result = sqlx::query(r#"
            INSERT INTO property
            (
                id, activation_time, address_line_1, address_line_2, ber_exempt,
                building_name, co_ownership_eligible, coming_soon, coming_soon_text,
                continuous_relisting, date_available_from, date_viewable_from, description,
                display_address, display_address_line_1, display_address_line_2, featurable,
                house_number, num_bedrooms, num_bathrooms, num_reception_rooms, url, postcode,
                price_tracker_available, property_type, published, region, sale_type, site_number,
                status, street, tag_line, time_activated_on_main_website, time_last_unpublished_on_p_p,
                town, unit_number
            ) VALUES 
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35,
                $36
            )
        "#
    ).bind(&property.path_id)
    .bind(&date_activated)
    .bind(&property.address_line_1)
    .bind(&property.address_line_2)
    .bind(&property.ber_exempt)
    .bind(&property.building_name)
    .bind(&property.co_ownership_eligible)
    .bind(&property.coming_soon)
    .bind(&property.coming_soon_text)
    .bind(&property.continuous_relisting)
    .bind(&date_available)
    .bind(&date_viewable)
    .bind(&property.description)
    .bind(&property.display_address)
    .bind(&property.display_address_line_1)
    .bind(&property.display_address_line_2)
    .bind(&property.featurable)
    .bind(&property.house_number)
    .bind(&property.num_bedrooms)
    .bind(&property.num_bathrooms)
    .bind(&property.num_reception_rooms)
    .bind(&url)
    .bind(&property.postcode)
    .bind(&property.price_tracker_available)
    .bind(&property.property_type.key)
    .bind(&property.published)
    .bind(&property.region)
    .bind(&property.sale_type.key)
    .bind(&property.site_number)
    .bind(&status)
    .bind(&property.street)
    .bind(&property.tag_line)
    .bind(&date_activated_on_site)
    .bind(&date_unpublished)
    .bind(&property.town)
    .bind(&property.unit_number)
    .execute(&mut *transaction)
    .await;
    handle_db_errors::<SqliteQueryResult, Error>(result, &property.path_id, "Property");
}

async fn insert_auction(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(auction) = &property.auction {
        let property_id = &property.path_id;
        let end_time = convert_date_to_string(&auction.end_time);
        let venue = match &auction.venue {
            Some(v) => {
                insert_venue(v, &mut *transaction).await;
                Some(v.id)
            },
            None => None
        };
        
        let result =sqlx::query(r#"
                INSERT INTO auction 
                (
                    property_id, end_time, lot, venue_id
                ) VALUES
                (
                    $1, $2, $3, $4
                )

            "#
        ).bind(&property_id)
        .bind(&end_time)
        .bind(&auction.lot)
        .bind(&venue)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "Auction");
    }
}

async fn insert_venue(venue: &Venue, transaction: &mut SqliteConnection) {
    let result = sqlx::query(r#"
            INSERT INTO venue 
            (
                id, address_line_1, address_line_2, name, online_only, postcode,
                region, town, url
            ) VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )

        "#
    ).bind(&venue.id)
    .bind(&venue.address_line_1)
    .bind(&venue.address_line_2)
    .bind(&venue.name)
    .bind(&venue.online_only)
    .bind(&venue.postcode)
    .bind(&venue.region)
    .bind(&venue.town)
    .bind(&venue.url)
    .execute(&mut *transaction)
    .await;
    handle_db_errors::<SqliteQueryResult, Error>(result, &venue.id.to_string(), "Venue");
    
}

async fn  insert_ber(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(b) = &property.ber {
        let result =sqlx::query(r#"
            INSERT INTO ber
            (
                property_id, alphanumeric_rating, energy_performance_indicator
            ) VALUES
            (
                $1, $2, $3
            )
        "#
        ).bind(property_id)
        .bind(&b.alphanumeric_rating)
        .bind(&b.energy_performance_indicator)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "ber");
    }   
}

async fn  insert_coordinate(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    let c = &property.coordinate;
    let result = sqlx::query(r#"
        INSERT INTO coordinate
        (
            property_id, accuracy, latitude, longitude
        ) VALUES
        (
            $1, $2, $3, $4
        )
    "#
    ).bind(property_id)
    .bind(&c.accuracy)
    .bind(&c.latitude)
    .bind(&c.longitude)
    .execute(&mut *transaction)
    .await;
    handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "coordinate");
}

async fn insert_country(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    let country = &property.country;
    let result = sqlx::query(
        r#"INSERT INTO country
        (
            property_id, iso_code_2, iso_code_3
        ) VALUES
        (
            $1, $2, $3
        )
    "#
    ).bind(&property_id)
    .bind(&country.iso_code_2)
    .bind(&country.iso_code_3)
    .execute(&mut *transaction)
    .await;
    handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "country");
}

async fn insert_epc(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(e) = &property.epc {
        let result = sqlx::query(r#"
            INSERT INTO epc
            (
                property_id, rating_shorthand
            ) VALUES 
            (
                $1, $2
            )
        "#
        ).bind(&property_id)
        .bind(&e.rating_shorthand)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "epc");
    }
}

async fn insert_furnished(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(f) =  &property.furnished_type {
        let result = sqlx::query(r#"
            INSERT INTO furnished
            (
                property_id, key, text
            ) VALUES 
            (
                $1, $2, $3
            )
        "#
        ).bind(&property_id)
        .bind(&f.key)
        .bind(&f.text)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "furnished");
    }
}

async fn insert_history(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(h) = &property.history {
        for history in h {
            let time_modified = convert_date_to_string(&history.time_modified);
            let result = sqlx::query(r#"
                INSERT INTO history
                (
                    property_id, time_modified, previous_published, published,
                    published_on_awp, published_on_awp_change, status_key, status_text, 
                    status_change
                ) VALUES
                (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9
                )
            "#
            ).bind(&property_id)
            .bind(&time_modified)
            .bind(&history.previous_published)
            .bind(&history.published)            
            .bind(&history.published_on_awp)
            .bind(&history.published_on_awp_change)
            .bind(&history.status.key)
            .bind(&history.status.text)
            .bind(&history.status_change)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "history");

            let result = sqlx::query(r#"
                INSERT INTO price
                (
                    property_id, price, date, change, prefix, suffix
                ) VALUES
                (
                    $1, $2, $3, $4, $5, $6
                )
            "#
            ).bind(&property_id)
            .bind(&history.price)
            .bind(&time_modified)
            .bind(&history.price_change)
            .bind(&history.price_prefix)
            .bind(&history.price_suffix)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "price");   
        }
    }
}

async fn insert_stat_history(stat_history: &Vec<StatHistory>, property_id: &str, transaction: &mut SqliteConnection) {
    for element in stat_history {
        let date = convert_date_to_string(&Some(element.date));
        let result = sqlx::query(r#"
            INSERT INTO stat_history
            (
                stat_id, bumped, date, featured, published, total_unique_views, total_views
            ) VALUES    
            (
                $1, $2, $3, $4, $5, $6, $7
            )
            "#
        ).bind(&property_id)
        .bind(&element.bumped)
        .bind(&date)
        .bind(&element.featured)
        .bind(&element.published)
        .bind(&element.total_unique_views)
        .bind(&element.total_views)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "stat_history");   
    }
}

async fn insert_ranking(ranking: &Ranking, property_id: &str, transaction: &mut SqliteConnection) {
    let (from, to) = match &ranking.range {  
        Some(r) => (Some(r.from), Some(r.to)),
        None => (None, None)
    };

    let popularity = match &ranking.popularity {
        Some(p) => Some(p),
        None => None
    };

    let term = match &ranking.term {
        Some(t) => Some(t),
        None => None
    };


    match (from, to, popularity, term) {
        (None, None, None, None) => (),
        (f, t, p, te) => {
            let result = sqlx::query(r#"
                INSERT OR IGNORE INTO ranking
                (
                    property_id, popularity, price_from, price_to, term
                ) VALUES
                (
                    $1, $2, $3, $4, $5
                )
            "#).bind(&property_id)
            .bind(&p)
            .bind(&f)
            .bind(&t)
            .bind(&te)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "ranking");
        }
    }
}

async fn insert_stat(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(stat) = &property.stats {
        let result = sqlx::query(r#"
            INSERT OR IGNORE INTO stat
            (
                property_id, total_views
            ) VALUES
            (
                $1, $2
            )
        "#).bind(&property_id)
        .bind(&stat.total_views)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "stat");   

        if let Some(ranking) = &stat.ranking {
            insert_ranking(ranking, property_id, transaction).await;
        }

        if let Some(stat_history) = &stat.history {
            insert_stat_history(stat_history, property_id, transaction).await;
        }
    }
}

async fn update_stat(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    if let Some(stat) = &property.stats {
        let result = sqlx::query(r#"
            UPDATE stat
            SET total_views = $1
            WHERE property_id = $2
        "#).bind(&stat.total_views)
        .bind(&property_id)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "stat");   

        if let Some(ranking) = &stat.ranking {
            insert_ranking(ranking, property_id, transaction).await;
        }

        if let Some(stat_history) = &stat.history {
            insert_stat_history(stat_history, property_id, transaction).await;
        }
    }
}

async fn insert_images(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(images) = &property.images {
        let property_id = &property.path_id;
        for image in images {
            let result = sqlx::query(r#"
                INSERT INTO image
                (
                    property_id, url, height, width, image_type
                ) VALUES    
                (
                    $1, $2, $3, $4, $5
                )
            "#).bind(property_id)
            .bind(&image.url)
            .bind(&image.height)
            .bind(&image.width)
            .bind(&image.image_type)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "image");   
        }
    }
}

async fn insert_key_info(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(information) = &property.key_info {
        let property_id = &property.path_id;
        for info in information {
            let result = sqlx::query(r#"
                INSERT INTO key_info
                (
                    property_id, name, key_info_group, text, subtext
                ) VALUES    
                (
                    $1, $2, $3, $4, $5
                )
            "#).bind(property_id)
            .bind(&info.name)
            .bind(&info.group)
            .bind(&info.text)
            .bind(&info.subtext)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "key_info");   
        }
    }
}

async fn insert_listing_update_time(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(time) = &property.listing_updated_time {
        let update_time = convert_date_to_string(&Some(*time));
        let property_id = &property.path_id;
        let result = sqlx::query(r#"
            INSERT INTO listing_update_time
            (
                property_id, listing_update_time
            ) VALUES    
            (
                $1, $2
            )
        "#).bind(property_id)
        .bind(&update_time)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "listing_update_time");   
    }
}

async fn insert_show_home(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(show_home) = &property.show_home_opening_time {
        let show_home_date = convert_date_to_string(&show_home.date);
        let property_id = &property.path_id;
        let days = &show_home.days;
        let mut days_string = String::new();
        days.into_iter()
            .for_each(|day| {
                days_string.push_str(&format!("{} ", &day.to_string()));
            });
        
        let days_string = days_string.trim();

        let result = sqlx::query(r#"
            INSERT INTO show_home
            (
                property_id, by_appointment, date, days, default_text, end_hour, text_key
            ) VALUES    
            (
                $1, $2, $3, $4, $5, $6, $7
            )
        "#).bind(property_id)
        .bind(&show_home.by_appointment)
        .bind(&show_home_date)
        .bind(&days_string)
        .bind(&show_home.default_text)
        .bind(&show_home.end_hour)
        .bind(&show_home.text_key)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "show_home");   
    }
}

async fn insert_broadband(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(broadband) = &property.ofcom_broadband {
        let property_id = &property.path_id;
        let time_cached = convert_date_to_string(&Some(broadband.time_last_cached));
        let result = sqlx::query(r#"
            INSERT INTO broadband
            (
                property_id, max_bb_predicted_down, max_bb_predicted_up, max_predicted_down, 
                max_predicted_up, max_sfbb_predicted_down, max_sfbb_predicted_up, 
                max_ufbb_predicted_down, max_ufbb_predicted_up, time_last_cached
            ) VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
        "#).bind(&property_id)
        .bind(&broadband.max_bb_predicted_down)
        .bind(&broadband.max_bb_predicted_up)
        .bind(&broadband.max_predicted_down)
        .bind(&broadband.max_predicted_up)
        .bind(&broadband.max_sfbb_predicted_down)
        .bind(&broadband.max_sfbb_predicted_up)
        .bind(&broadband.max_ufbb_predicted_down)
        .bind(&broadband.max_ufbb_predicted_up)
        .bind(&time_cached)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "broadband");   
    }
}

async fn insert_style(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(style) = &property.style {
        let result = sqlx::query(r#"
            INSERT INTO style
            (
                key, property_type, text
            ) VALUES    
            (
                $1, $2, $3
            )
        "#
        ).bind(&style.key)
        .bind(&style.property_type)
        .bind(&style.text)
        .execute(&mut *transaction)
        .await;
        
        handle_db_errors::<SqliteQueryResult, Error>(result, &style.key, "style");   

        let property_id = &property.path_id;
        let result = sqlx::query(r#"
            INSERT INTO style_property
            (
                property_id, style_id
            ) VALUES    
            (
                $1, $2
            )
        "#
        ).bind(&property_id)
        .bind(&style.key)
        .execute(&mut *transaction)
        .await;
        handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "style_property");   
    }
}

async fn insert_style_group(property: &Property, transaction: &mut SqliteConnection) {
    if let Some(style_groups) = &property.style_groups {
        for style_group in style_groups {
            let result = sqlx::query(r#"
                INSERT OR IGNORE INTO style_group
                (
                    id, key, text
                ) VALUES    
                (
                    $1, $2, $3
                )
            "#
            ).bind(&style_group.id)
            .bind(&style_group.key)
            .bind(&style_group.text)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &style_group.id.to_string(), "style_group");   

            let property_id = &property.path_id;

            let result = sqlx::query(r#"
                INSERT INTO style_group_property
                (
                    property_id, style_group_id
                ) VALUES    
                (
                    $1, $2
                )
            "#
            ).bind(&property_id)
            .bind(&style_group.id)
            .execute(&mut *transaction)
            .await;
            handle_db_errors::<SqliteQueryResult, Error>(result, &property_id, "style_group_property");   
        }
    }
}

async fn update_status(property: &Property, transaction: &mut SqliteConnection) {
    let property_id = &property.path_id;
    let old_status = sqlx::query(r#"
        SELECT status AS old_status FROM property WHERE id = $1
    "#).bind(property_id)
    .fetch_one(&mut *transaction)
    .await.expect(&format!("Failed to retrieve status: {}", property_id));

    let old_status: String = old_status.get("old_status");

    match &property.status {
        Some(s) => {
            if old_status != s.key {
                sqlx::query(r#"
                    UPDATE property
                    SET status = $1
                    WHERE id = $2
                "#).bind(&s.key)
                .bind(property_id)
                .execute(&mut *transaction)
                .await
                .expect(&format!("Failed to update property status: {}", property_id));
            }
        },
        None => (),
    }
    
}

pub async fn updates(property: &Property, transaction: &mut SqliteConnection) {
    update_status(&property, transaction).await;
    update_stat(&property, transaction).await;
}

pub async fn dead_inserts(property: &Property, transaction: &mut SqliteConnection, stc: bool, unpublished: bool, stc_specific: Option<String>) {
    let property_id = &property.path_id;
    sqlx::query(r#"
        INSERT OR IGNORE INTO dead_property
        (
            property_id, unpublished, stc, stc_specific
        ) VALUES
        (
            $1, $2, $3, $4
        )
    "#).bind(&property_id)
    .bind(unpublished)
    .bind(stc)
    .bind(&stc_specific)  
    .execute(&mut *transaction)
    .await
    .expect("Failed to insert dead property");
}

pub async fn healthy_inserts(property: &Property, account: &Account, transaction: &mut SqliteConnection) {
    insert_property(property, transaction).await;
    insert_auction(property, transaction).await;
    insert_account(account, transaction).await;
    insert_award(account, transaction).await;
    insert_accreditation(&account.accreditations, &account.account_number, transaction, true).await;
    insert_accreditation(&account.missing_accreditations, &account.account_number,transaction, false).await;
    insert_tour(property, transaction).await;
    insert_ber(property, transaction).await;
    insert_coordinate(property, transaction).await;
    insert_country(property, transaction).await;
    insert_epc(property, transaction).await;
    insert_furnished(property, transaction).await;
    insert_history(property, transaction).await;
    insert_stat(property, transaction).await;
    insert_images(property, transaction).await;
    insert_key_info(property, transaction).await;
    insert_listing_update_time(property, transaction).await;
    insert_show_home(property, transaction).await;
    insert_broadband(property, transaction).await;
    insert_style(property, transaction).await;
    insert_style_group(property, transaction).await;
}

pub async fn select(transaction: &mut SqliteConnection) -> Vec<serializers::Property> {
    let result = sqlx::query(r#"
        SELECT * FROM property
    "#).fetch_all(&mut *transaction).await.expect("What the fuck");
    let format = "%Y-%m-%d %H:%M:%S";
    // Parse the string into a NaiveDateTime object
    
    let result = result.iter()
        .map(|row| {
            let activation_time = match chrono::DateTime::<FixedOffset>::parse_from_str(row.get("activation_time"), format) {
                Ok(a_t) => Some(a_t),
                Err(_) => None,
            };
            let date_available_from = match chrono::DateTime::<FixedOffset>::parse_from_str(row.get("date_available_from"), format) {
                Ok(a_t) => Some(a_t),
                Err(_) => None,
            };
            let date_viewable_from = match chrono::DateTime::<FixedOffset>::parse_from_str(row.get("date_viewable_from"), format) {
                Ok(a_t) => Some(a_t),
                Err(_) => None,
            };
            let time_activated_on_main_website = match chrono::DateTime::<FixedOffset>::parse_from_str(row.get("time_activated_on_main_website"), format) {
                Ok(a_t) => Some(a_t),
                Err(_) => None,
            };
            let time_last_unpublished_on_p_p = match chrono::DateTime::<FixedOffset>::parse_from_str(row.get("time_last_unpublished_on_p_p"), format) {
                Ok(a_t) => Some(a_t),
                Err(_) => None,
            };

            let id: u32 = row.get("id");
            let path_id: String = format!("{}", id);
            
            Property {
                id,
                tour: None,
                account: None,
                auction: None, 
                ber: None, 
                brief_text: None, 
                coordinate: Coordinate {
                    accuracy: None, 
                    latitude: None, 
                    longitude: None
                },
                country: Country {
                    iso_code_2: None, 
                    iso_code_3: None, 
                    name: None  
                },
                epc: None,
                furnished_type: None,
                history: None, 
                images: None, 
                key_info: None, 
                listing_updated_time: None, 
                ofcom_broadband: None, 
                path: String::from("as"), 
                path_id,
                price: None, 
                show_home_opening_time: None, 
                stats: None, 
                style: None, 
                style_groups: None,
                activation_time,
                address_line_1: row.get("address_line_1"),
                address_line_2: row.get("address_line_1"),
                ber_exempt: row.get("ber_exempt"),
                building_name: row.get("building_name"),
                co_ownership_eligible: row.get("co_ownership_eligible"),
                coming_soon: row.get("coming_soon"),
                coming_soon_text: row.get("coming_soon_text"),
                continuous_relisting: row.get("continuous_relisting"),
                date_available_from,
                date_viewable_from,
                description: row.get("description"),
                display_address: row.get("display_address"),
                display_address_line_1: row.get("display_address_line_1"),
                display_address_line_2: row.get("display_address_line_2"),
                featurable: row.get("featurable"),
                house_number: row.get("house_number"),
                num_bedrooms: row.get("num_bedrooms"),
                num_bathrooms: row.get("num_bathrooms"),
                num_reception_rooms: row.get("num_reception_rooms"),
                postcode: row.get("postcode"),
                price_tracker_available: row.get("price_tracker_available"),
                property_type: PropertyType {
                    key: String::from("test"),
                    text: String::from("test")
                },
                published: row.get("published"),
                region: row.get("region"),
                sale_type: SaleType {
                    key: String::from("test"),
                    text: String::from("test")
                },
                site_number: row.get("site_number"),
                status: Some(Status {
                    key: String::from("test"),
                    text: String::from("test")
                }),
                street: row.get("street"),
                tag_line: row.get("tag_line"),
                time_activated_on_main_website,
                time_last_unpublished_on_p_p,
                town: row.get("town"),
                unit_number: row.get("unit_number"),
            }
        }).collect();

    result
}   