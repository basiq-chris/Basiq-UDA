use std::{fs, str, time::SystemTime};
use reqwest::{self, header};
use serde_json::Value;

pub fn get() -> (String, u64) {
    let key = read_key();
    let req =  reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/token")
    .header(header::AUTHORIZATION, "Basic ".to_owned() + &key)
    .header(header::ACCEPT, "application/json")
    .header(header::CONTENT_TYPE, "application/x-www-form-url")
    .send();
    

    match req {
        Ok(val) => {
            match val.status().as_u16() {
                200 => {
                    let tkn: Value = serde_json::from_str(val.text().unwrap().as_str()).unwrap();
                    return (tkn["access_token"].as_str().map(String::from).unwrap(), SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + tkn["expires_in"].as_u64().unwrap_or_else(|| 3600))
                },
                400 => {panic!("400")},
                404 => {panic!("404")},
                500 => {panic!("500")}
                _ => panic!("Unknown Error Contact package maintainer")
            }
        }
        Err(_) => {
            panic!("Something went wrong, look in logger for extra details");
        }
    }
}

fn read_key() -> String {
    let reader = str::from_utf8(&fs::read("./APIKEY.env").unwrap_or_else(|x| panic!("Rename your APIKEY.env.dst to APIKEY.env and try again\n\nError: {}", x))).unwrap().to_string();
    if reader.len() != 101 {
        panic!("Invalid API Key");
    }
    if reader.eq("PUTAPIKEYHERE") {
        panic!("Put the API key in APIKEY.env");
    }
    reader
}