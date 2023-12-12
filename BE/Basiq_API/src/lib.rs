use std::time::{SystemTime, UNIX_EPOCH};

pub mod requestHandler;

pub enum RequestType {
    //API KEY, KeyType
    Token(String, KeyType),
    //ID/email, mobile, fname, mname, lname
    Users(Vec<String>),
    //ID
    Consent(String),
    //ID/userID
    AuthLink(String),
    //ID/userID
    Jobs(String)
}

#[allow(non_camel_case_types)]
pub enum KeyType {
    SERVER_ACCESS,
    CLIENT_ACCESS
}

pub struct Token {
    pub token: String,
    expiry: u64
}

impl Token {
    pub fn has_expired(&self) -> bool {
        return self.expiry < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
}