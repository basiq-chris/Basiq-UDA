use SXL::{Log, Token, User};
use reqwest::{self, header};
use serde_json;

pub fn create(user: User, tkn: &Token) -> Log {
    if user.email.is_none() && user.mobile.is_none() {
        panic!("Must have at least email or phone number");
    }

    let req = reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/users")
    .bearer_auth(tkn.token.clone())
    .header(header::CONTENT_TYPE, "application/json")
    .header(header::ACCEPT, "application/json")
    .json(serde_json::Value::String(format!(r#"
        {{
            "email": {},
            "mobile": {},
            "firstName": {},
            "middleName": {},
            "lastName": {}
        }}
    "#, user.email.unwrap_or_default(), user.mobile.unwrap_or_default(), user.first_name.unwrap_or_default(), user.middle_name.unwrap_or_default(), user.last_name.unwrap_or_default())).as_object().unwrap());

    let requ = SXL::UserRequest {
        request_data: req,
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: user
    };

    let resp = requ.request_data.send().unwrap();

}
