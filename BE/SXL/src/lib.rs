use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{StatusCode, RequestBuilder, Method, header::HeaderMap};
use serde_json::Value;

pub enum RequestType {
    User,
    Token,
    Consent,
    AuthLink,
}

pub enum Direction {
    Request,
    Response,
}

pub trait SXLoggableRequest {
    fn get_verb(&self) -> reqwest::Method;
    fn get_headers(&self) -> reqwest::header::HeaderMap;
    fn get_data(&self) -> String;
    fn send(&self) -> reqwest::blocking::Response;
}

pub trait SXLoggableResponse {
    fn get_status(&self) -> reqwest::StatusCode;
    fn get_headers(&self) -> reqwest::header::HeaderMap;
    fn get_data(&self) -> String;
}

pub struct Log {
    pub req_t: RequestType,
    pub request: Box<dyn SXLoggableRequest>,
    pub response: Box<dyn SXLoggableResponse>,
}

#[derive(Default)]
pub struct LogQueue {
    pub queue: std::collections::VecDeque<Log>,
}

impl LogQueue {
    pub fn new() -> Self {
        LogQueue {
            queue: std::collections::VecDeque::new(),
        }
    }

    pub fn push(log: Log, q: &mut Self) {
        q.queue.push_back(log)
    }

    pub fn pull(q: &mut Self) -> Result<Log, u8> {
        match q.queue.pop_front() {
            Some(val) => Ok(val),
            None => Err(0),
        }
    }
}

//TOKENS
pub struct Token {
    token: String,
    expires: u64,
}

impl Token {
    pub fn from_raw_json(json: serde_json::Value) -> Self {
        let access = json["access_token"].as_str().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + json["expires_in"].as_u64().unwrap();

        Token {
            token: access.to_string(),
            expires: timestamp,
        }
    }

    pub fn get_data(&self) -> String {
        String::from("T:".to_owned() + &self.token.to_string() + "E:" + &self.expires.to_string())
    }

    pub fn get_token(&self) -> Result<String, String> {
        if self.expires < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
            return Err("Expired".to_string());
        }
        Ok(self.token.clone())
    }
}

pub struct TokenRequest {
    pub isdev: bool,
    pub request_data: reqwest::blocking::RequestBuilder,
    pub api_key: String,
}

impl TokenRequest {
    pub fn dev_switch(mut self) -> Self {
        self.isdev = true;
        self
    }
}

impl SXLoggableRequest for TokenRequest {
    fn get_verb(&self) -> reqwest::Method {
        self.request_data
            .try_clone()
            .unwrap()
            .build()
            .unwrap()
            .method()
            .clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.request_data
            .try_clone()
            .unwrap()
            .build()
            .unwrap()
            .headers()
            .clone()
    }

    fn get_data(&self) -> String {
        if self.isdev {
            return String::from("IF YOU NEED THE API KEY ASK THE PACKAGE MAINTAINER");
        }
        self.api_key.clone()
    }

    fn send(&self) -> reqwest::blocking::Response {
        match self.request_data.try_clone().unwrap().send() {
            Ok(val) => val,
            Err(_) => panic!("Request cannot be made"),
        }
    }
}

pub struct TokenResponse {
    pub response_data: reqwest::blocking::Response,
    pub token: Token,
}

impl TokenResponse {
    pub fn new(mut response: reqwest::blocking::Response) -> Self {
        let json;
        let mut buf: Vec<u8> = vec![];
        let _ = response.copy_to(&mut buf);
        json = std::str::from_utf8(buf.as_ref()).unwrap();
        TokenResponse {
            response_data: response,
            token: Token::from_raw_json(serde_json::from_str(json).unwrap()),
        }
    }

    pub fn get_token(&self) -> String {
        match self.token.get_token() {
            Ok(val) => val,
            Err(_) => panic!("Humongous lag what?!")
        }
    }
}

impl SXLoggableResponse for TokenResponse {
    fn get_status(&self) -> reqwest::StatusCode {
        self.response_data.status()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.response_data.headers().clone()
    }

    fn get_data(&self) -> String {
        self.token.get_data()
    }
}

//USERS
#[derive(Clone)]
pub struct User {
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

impl User {
    pub fn get_data(&self) -> String {
        let usrdata = self.clone();
        String::from(
            format!(r#"
            {{
                "email":"{}",
                "mobile":"{}",
                "first_name":"{}",
                "middle_name":"{}",
                "last_name":"{}"
            }}
            "#, usrdata.email.unwrap_or_default(), usrdata.mobile.unwrap_or_default(), usrdata.first_name.unwrap_or_default(), usrdata.middle_name.unwrap_or_default(), usrdata.last_name.unwrap_or_default())
        )
    }
}

pub struct UserRequest {
    pub request_data: reqwest::blocking::RequestBuilder,
    pub verb: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
    pub data: String,
}

impl SXLoggableRequest for UserRequest {
    fn get_verb(&self) -> reqwest::Method {
        self.verb.clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }

    fn send(&self) -> reqwest::blocking::Response {
        match self.request_data.try_clone().unwrap().send() {
            Ok(val) => val,
            Err(_) => panic!("Unable to send request"),
        }
    }
}

pub struct UserResponse {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub data: Value,
}

impl UserResponse {
    pub fn new(response: reqwest::blocking::Response) -> Self {
        UserResponse {
            status: response.status(),
            headers: response.headers().clone(),
            data: serde_json::from_str(response.text().unwrap().as_str()).unwrap(),
        }
    }
}

impl SXLoggableResponse for UserResponse {
    fn get_status(&self) -> reqwest::StatusCode {
        self.status
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    fn get_data(&self) -> String {
        self.data.to_string()
    }
}

//Auth_Link
pub struct AuthLink {
    pub data: Value
}

impl AuthLink {
    pub fn serialise(raw: &str) -> Self {
        AuthLink { data: serde_json::from_str(raw).unwrap() }
    }
}

pub struct AuthLinkRequest {
    pub request_data: reqwest::blocking::RequestBuilder,
    pub verb: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
    pub data: String
}

impl SXLoggableRequest for AuthLinkRequest {
    fn get_verb(&self) -> reqwest::Method {
        self.verb.clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }

    fn send(&self) -> reqwest::blocking::Response {
        self.request_data.try_clone().unwrap().send().unwrap()
    }
}

pub struct AuthLinkResponse {
    pub status: StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub data: AuthLink
}

impl SXLoggableResponse for AuthLinkResponse {
    fn get_status(&self) -> reqwest::StatusCode {
        self.status.clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    fn get_data(&self) -> String {
        self.data.data.as_str().map(String::from).unwrap()
    }
}

//Consents
pub struct Consent {
    pub data: Value
}

pub struct ConsentRequest {
    pub request_data: reqwest::blocking::RequestBuilder,
    pub verb: Method,
    pub headers: HeaderMap,
    pub data: String
}

impl SXLoggableRequest for ConsentRequest {
    fn get_verb(&self) -> reqwest::Method {
        self.verb.clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }

    fn send(&self) -> reqwest::blocking::Response {
        self.request_data.try_clone().unwrap().send().unwrap()
    }
}

pub struct ConsentResponse {
    pub status: StatusCode,
    pub header: HeaderMap,
    pub data: Consent
}

impl SXLoggableResponse for ConsentResponse {
    fn get_status(&self) -> reqwest::StatusCode {
        self.status.clone()
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.header.clone()
    }

    fn get_data(&self) -> String {
        self.data.data.to_string()
    }
}