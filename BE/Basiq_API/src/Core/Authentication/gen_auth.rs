use std::fs;
use SXL::{Log, TokenResponse, SXLoggableRequest};
use reqwest::{self, header};

pub fn get(thread_client: reqwest::blocking::Client) -> (Log, String) {
    let key = read_key();
    let req =  reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/token")
    .header(header::AUTHORIZATION, "Basic ".to_owned() + &key)
    .header(header::ACCEPT, "application/json")
    .header(header::CONTENT_TYPE, "application/x-www-form-url");

    let tr = SXL::TokenRequest {
        isdev: true,
        request_data: req,
        api_key: key
    };

    let tR: TokenResponse;
    {
        let resp = tr.send();
        tR = TokenResponse::new(resp);
    }
    
    let token_string = tR.get_token();
    

    return (Log {
        req_t: SXL::RequestType::Token,
        request: Box::new(tr.dev_switch()),
        response: Box::new(tR)
    }, token_string);


}

fn read_key() -> String {
    let reader = std::str::from_utf8(&fs::read("./APIKEY.env").unwrap_or_else(|x| panic!("Rename your APIKEY.env.dst to APIKEY.env and try again\n\nError: {}", x))).unwrap().to_string();
    if reader.len() != 101 {
        panic!("Invalid API Key");
    }
    if reader.eq("PUTAPIKEYHERE") {
        panic!("Put the API key in APIKEY.env");
    }
    reader
}