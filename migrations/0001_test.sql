CREATE TABLE property (
    id INTEGER PRIMARY KEY, -- explicitly created 
    activation_time TEXT NOT NULL,
    address_line_1 VARCHAR(100) NOT NULL,
    address_line_2 VARCHAR(100) NOT NULL,
    ber_exempt BOOLEAN NOT NULL,
    building_name VARCHAR(100) NOT NULL,
    co_ownership_eligible BOOLEAN NOT NULL,
    coming_soon BOOLEAN NOT NULL, 
    coming_soon_text VARCHAR(30),
    continuous_relisting BOOLEAN NOT NULL,
    date_available_from TEXT,
    date_viewable_from TEXT,
    description TEXT NOT NULL,
    display_address VARCHAR(200) NOT NULL,
    display_address_line_1 VARCHAR(200) NOT NULL,
    display_address_line_2 VARCHAR(200) NOT NULL,
    featurable BOOLEAN NOT NULL,
    house_number VARCHAR(10) NOT NULL,
    num_bedrooms INTEGER, 
    num_bathrooms INTEGER,
    num_reception_rooms INTEGER,
    url VARCHAR(200) NOT NULL,
    postcode VARCHAR(10) NOT NULL,
    price_tracker_available BOOLEAN NOT NULL,
    property_type VARCHAR(20) NOT NULL,
    published BOOLEAN NOT NULL,
    region VARCHAR(50) NOT NULL,
    sale_type VARCHAR(20) NOT NULL,
    site_number VARCHAR(20) NOT NULL,
    status VARCHAR(10),
    street VARCHAR(100) NOT NULL,
    tag_line TEXT,
    time_activated_on_main_website TEXT,
    time_last_unpublished_on_p_p TEXT,
    town VARCHAR(50),
    unit_number VARCHAR(50)
);

CREATE TABLE dead_property (
    property_id INTEGER PRIMARY KEY, 
    unpublished BOOLEAN NOT NULL DEFAULT FALSE,
    stc BOOLEAN NOT NULL DEFAULT FALSE,
    stc_specific VARCHAR(12)
);

CREATE TABLE tour (
    property_id INTEGER,
    url VARCHAR(70) NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, url)
);

CREATE TABLE auction (
    property_id INTEGER PRIMARY KEY, 
    end_time VARCHAR(50),
    lot VARCHAR(3) NOT NULL,
    venue_id INTEGER,
    FOREIGN KEY (property_id) REFERENCES property(id),
    FOREIGN KEY (venue_id) REFERENCES venue(id)
);

CREATE TABLE venue (
    id INTEGER PRIMARY KEY,
    address_line_1 VARCHAR(100) NOT NULL,
    address_line_2 VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    online_only BOOLEAN NOT NULL,
    postcode VARCHAR(10) NOT NULL,
    region VARCHAR(100) NOT NULL,
    town VARCHAR(100) NOT NULL,
    url VARCHAR(100) NOT NULL
);

CREATE TABLE ber (
    property_id INTEGER PRIMARY KEY,
    alphanumeric_rating VARCHAR(4),
    energy_performance_indicator DECIMAL(10, 2),
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE coordinate (
    property_id INTEGER PRIMARY KEY,
    accuracy VARCHAR(20),
    latitude DECIMAL(9,6),
    longitude DECIMAL(9,6)
);

CREATE TABLE country (
    property_id INTEGER PRIMARY KEY,
    iso_code_2 VARCHAR(20),
    iso_code_3 VARCHAR(20),
    name VARCHAR(20)
);

CREATE TABLE epc (
    property_id INTEGER PRIMARY KEY,
    rating_shorthand VARCHAR(8) NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE furnished (
    property_id INTEGER PRIMARY KEY,    
    key VARCHAR(20) NOT NULL,
    text VARCHAR(20) NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property_id INTEGER NOT NULL,
    time_modified VARCHAR(50),
    previous_published BOOLEAN NOT NULL,
    published BOOLEAN NOT NULL,
    published_on_awp BOOLEAN NOT NULL,
    published_on_awp_change BOOLEAN NOT NULL,
    status_key VARCHAR(20) NOT NULL,
    status_text VARCHAR(20) NOT NULL,
    status_change BOOLEAN NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, time_modified)
);

CREATE TABLE stat (
    property_id INTEGER PRIMARY KEY,
    total_views INTEGER NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE ranking (
    property_id INTEGER NOT NULL,
    popularity VARCHAR(10),
    price_from INTEGER,
    price_to INTEGER,
    term VARCHAR(50),
    date TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, popularity)
);

CREATE TABLE stat_history (
    stat_id INTEGER NOT NULL,
    bumped BOOLEAN NOT NULL,
    date TEXT NOT NULL,
    featured BOOLEAN NOT NULL,
    published BOOLEAN  NOT NULL,
    total_unique_views INTEGER NOT NULL,
    total_views INTEGER NOT NULL,
    FOREIGN KEY (stat_id) REFERENCES stat(property_id),
    UNIQUE(stat_id, date)
);

CREATE TABLE image (
    property_id INTEGER NOT NULL,
    url VARCHAR(200) NOT NULL,
    height INTEGER NOT NULL,
    width INTEGER NOT NULL,
    image_type VARCHAR(100) NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE(property_id, url)
);

CREATE TABLE key_info (
    property_id INTEGER NOT NULL,
    name VARCHAR(20) NOT NULL,
    key_info_group VARCHAR(20) NOT NULL,
    text VARCHAR(20),
    subtext VARCHAR(20),
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, name)
);

CREATE TABLE listing_update_time (
    property_id INTEGER NOT NULL, 
    listing_update_time TEXT NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, listing_update_time)
);

CREATE TABLE show_home (
    property_id INTEGER PRIMARY KEY, 
    by_appointment BOOLEAN NOT NULL,
    date TEXT,
    days TEXT NOT NULL,
    default_text TEXT NOT NULL,
    end_hour INTEGER,
    text_key TEXT NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE broadband (
    property_id INTEGER PRIMARY KEY,
    max_bb_predicted_down DECIMAL(6,2),
    max_bb_predicted_up DECIMAL(6,2),
    max_predicted_down DECIMAL(6,2),
    max_predicted_up DECIMAL(6,2),
    max_sfbb_predicted_down DECIMAL(6,2),
    max_sfbb_predicted_up DECIMAL(6,2),
    max_ufbb_predicted_down DECIMAL(6,2),
    max_ufbb_predicted_up DECIMAL(6,2),
    time_last_cached TEXT NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id)
);

CREATE TABLE price (
    property_id INTEGER NOT NULL,
    price INTEGER NOT NULL, 
    date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    change BOOLEAN NOT NULL,
    prefix VARCHAR(20),
    suffix VARCHAR(20),
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (property_id, date)
);

CREATE TABLE style_property (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property_id INTEGER NOT NULL,
    style_id VARCHAR(50) NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    FOREIGN KEY (style_id) REFERENCES style(key),
    UNIQUE (property_id, style_id)
);

CREATE TABLE style (
    key VARCHAR(70) PRIMARY KEY,
    property_type VARCHAR(70) NOT NULL,
    text VARCHAR(70) NOT NULL
);

CREATE TABLE style_group_property(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property_id INTEGER NOT NULL,
    style_group_id INTEGER NOT NULL,
    FOREIGN KEY (property_id) REFERENCES property(id),
    FOREIGN KEY (style_group_id) REFERENCES style_group(id),
    UNIQUE(property_id, style_group_id)
);

CREATE TABLE style_group (
    id INTEGER PRIMARY KEY,
    key VARCHAR(50) NOT NULL,
    text VARCHAR(50) NOT NULL
);

CREATE TABLE property_account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property_id INTEGER NOT NULL,
    account_id VARCHAR(12) NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (property_id) REFERENCES property(id),
    UNIQUE (account_id, property_id)
);

CREATE TABLE account (
    id VARCHAR(12) PRIMARY KEY, -- explicitly created
    agent BOOLEAN NOT NULL,
    developer BOOLEAN NOT NULL, 
    display_address VARCHAR(100),
    enchanced_branding BOOLEAN NOT NULL,
    include_fee_charges BOOLEAN NOT NULL,
    phone VARCHAR(20),
    organisation VARCHAR(100) NOT NULL,
    organisation_branch VARCHAR(100) NOT NULL,
    private_developer BOOLEAN NOT NULL,
    psr_license_number VARCHAR(12),
    tier VARCHAR(10) NOT NULL,
    website_url VARCHAR(100)
);

CREATE TABLE accreditation_account (
    account_id VARCHAR(12) NOT NULL,
    accreditation_id INTEGER NOT NULL,
    have BOOLEAN NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (accreditation_id) REFERENCES accreditation(id),
    UNIQUE (account_id, accreditation_id)
);

CREATE TABLE accreditation (
    id INTEGER PRIMARY KEY,
    label VARCHAR(20) NOT NULL,
    text_key VARCHAR(100) NOT NULL,
    tooltip VARCHAR(100) NOT NULL,
    accreditation_type VARCHAR(100) NOT NULL,
    url VARCHAR(200) NOT NULL
);

CREATE TABLE award_account (
    account_id VARCHAR(12) NOT NULL,
    award_id INTEGER NOT NULL,
    have BOOLEAN NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (award_id) REFERENCES award(id),
    UNIQUE (account_id, award_id)
);

CREATE TABLE award (
    id INTEGER PRIMARY KEY,
    label VARCHAR(20) NOT NULL,
    text_key VARCHAR(100) NOT NULL,
    tooltip VARCHAR(100) NOT NULL,
    award_type VARCHAR(100) NOT NULL,
    url VARCHAR(200) NOT NULL
); 