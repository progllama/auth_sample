use csv::Writer;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use actix_web::HttpResponse;
use actix_web::http;
use rand::RngCore;
use super::*;

pub fn save_user(user: User) {
    let file = std::fs::OpenOptions::new()
        .append(true)
        .open(consts::USER_DATA_PATH).unwrap();
    let mut writer = Writer::from_writer(file);
    let _ = writer.serialize(user);
    let _ = writer.flush();
}

pub fn get_users() -> Vec<User> {
    let mut reader = csv::Reader::from_path(consts::USER_DATA_PATH).unwrap();
    reader.deserialize::<User>()
        .map(|result| {
            result.ok().unwrap()
        })
        .collect::<_>()
}

pub fn publish_token() -> String {
    let mut buf = [0u8; 256 / 8];
    let mut gen = rand::thread_rng();
    gen.fill_bytes(&mut buf);
    base64::encode(buf)
}

// TODO more strech.
pub fn hash(password: String) -> u64 {
    let mut s = DefaultHasher::new();
    password.hash(&mut s);
    s.finish()
}

pub fn redirect_to(path: &str) -> HttpResponse {
    HttpResponse::build(http::StatusCode::SEE_OTHER).header(http::header::LOCATION, path.clone()).finish()
}

pub fn find_user(email: String) -> Option<User> {
    let mut reader = csv::Reader::from_path(consts::USER_DATA_PATH).unwrap();
    let users = reader.deserialize::<User>();


    for user in users {
        if let Ok(user) = user {
            if user.email == email {
                return Some(user);
            }
        }
    }

    None
}