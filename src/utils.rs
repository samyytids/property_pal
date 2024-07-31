use sqlx::Error;
use chrono::{DateTime, FixedOffset};
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::serializers;
use sqlx::{Sqlite, Transaction};

pub fn delete_and_create_db() {
    match fs::remove_file("db.sqlite") {
        Ok(_) => println!("Deleted db.sqlite"),
        Err(_) => println!("db.sqlite doesn't exist, so no delete")
    }
    File::create("db.sqlite").expect("Failed to create db.sqlite");
}

pub fn handle_unwrapping_some<T>(option: Option<T>, message: &str) -> T {
    let result = match option {
        Some(r) => r,
        None => panic!("{}", message)
    };
    result 
}

pub fn handle_unwrapping_result<T, E>(option: Result<T, E>, message: &str) -> T {
    let result = match option {
        Ok(r) => r,
        Err(_) =>  panic!("{}", message)
    };
    result 
}

pub fn load_json_data() -> Vec<serializers::Data> {
    let mut file = File::open("test.json").unwrap();
    let mut data =  String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Vec<serializers::Data> = serde_json::from_str(&data).unwrap();
    json
}

pub async fn create_db_managers() -> Transaction<'static, Sqlite> {
    let pool = sqlx::SqlitePool::connect("./db.sqlite").await.expect("database connection failed: likely an incorrect path. Ensure db file is called db.sqlite");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Migrations failed");
    let transaction = pool.begin().await.expect("transaction failed to begin");
    transaction
}

pub fn split_vector<T>(vector: &[T], chunk_size: usize) -> Vec<Vec<T>> 
    where
        T: Clone
    {
    let result_vector = vector.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    result_vector
}

pub fn handle_db_errors<T, E>(result: Result<T, Error>, property_id: &str, table: &str) {
    match result {
        Ok(_) => (),
        Err(e) =>  {
            let message = e.as_database_error().unwrap().message();
            let code = &e.as_database_error().unwrap().code().unwrap();
            if code == "1555" || code == "2067" {
                ()
            }
            else {
                panic!("Something went wrong: {}, {}, {}, {}", code, message, property_id, table);
            }
        }
    }
}

pub fn convert_date_to_string(date: &Option<DateTime<FixedOffset>>) -> Option<String> {
    let result = match &date {
        Some(d) => Some(d.to_string()),
        None => None
    };
    result
}