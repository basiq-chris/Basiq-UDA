use std::time::{SystemTime, UNIX_EPOCH};

pub mod request_handler;

pub enum RequestType {
    //KeyType
    Token(KeyType),
    //ID/email, mobile, fname, mname, lname
    Users(Vec<String>),
    //ID
    Consent(String),
    //ID/userID
    AuthLink(String),
    //ID/userID
    Jobs(String),
    //UserID
    Accounts(String),
    //UserID
    Transactions(String)
}

#[allow(non_camel_case_types)]
pub enum KeyType {
    SERVER_ACCESS,
    CLIENT_ACCESS
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token: String,
    expiry: u64
}

impl Token {
    pub fn has_expired(&self) -> bool {
        return self.expiry < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    pub fn new(data: Vec<Box<(String, String)>>) -> Self {
        Token {
            token: data[1].1.to_string(),
            expiry: data[0].1.parse::<u64>().unwrap()
        }
    }
}