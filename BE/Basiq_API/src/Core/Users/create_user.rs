use SXL::Log;
use reqwest::{self, header};
use serde_json;
use crate::{Token, User};

pub fn create<Req: SXL::SXLoggableRequest, Res: SXL::SXLoggableResponse>(user: User, tkn: &Token) -> Log<Req, Res> {
    if user.email.is_none() && user.phone_number.is_none() {
        panic!("Must have at least email or phone number");
    }

    let req = reqwest::blocking::Client::new()
    .post("https://au-api.basiq.io/users")
    .bearer_auth(tkn.val.clone())
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
    "#, user.email.unwrap_or_default(), user.phone_number.unwrap_or_default(), user.first_name.unwrap_or_default(), user.middle_name.unwrap_or_default(), user.last_name.unwrap_or_default())).as_object().unwrap());

    return SXL::Log::send_and_log(req, SXL::RequestType::User);
}
