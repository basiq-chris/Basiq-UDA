use reqwest::{self, header};
use serde_json;
use crate::{Token, User};

pub fn create(user: User, tkn: &Token) {
    if user.email.is_none() && user.phone_number.is_none() {
        panic!("Must have at least email or phone number");
    }

    let req = reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/users")
    .bearer_auth(tkn.val)
    .header(header::CONTENT_TYPE, "application/json")
    .header(header::ACCEPT, "application/json")
    .json(serde_json::from_str(format!(r#"
        {{
            "email": {},
            "mobile": {},
            "firstName": {},
            "middleName": {},
            "lastName": {}
        }}
    "#, )))
}