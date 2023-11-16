use std::{fs, str, time::SystemTime};
use SXL::Log;
use reqwest::{self, header};
use serde_json::Value;

pub fn get<Res: SXL::SXLoggableResponse, Req: SXL::SXLoggableRequest>() -> Log<Req, Res> {
    let key = read_key();
    let req =  reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/token")
    .header(header::AUTHORIZATION, "Basic ".to_owned() + &key)
    .header(header::ACCEPT, "application/json")
    .header(header::CONTENT_TYPE, "application/x-www-form-url")
    .build();


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