use SXL::{Log, User};
use reqwest::{self, header::ACCEPT};
use serde_json;

pub fn create(user: User, tkn: String, thread_client: reqwest::blocking::Client) -> Log {
    if user.email.is_none() && user.mobile.is_none() {
        panic!("Must have at least email or phone number");
    }
    let usrcpy = user.clone();
    let req = thread_client.post("https://au-api.basiq.io/users")
    .bearer_auth(tkn)
    .json(
        serde_json::Value::String(format!(r#"
        {{
            "email": {},
            "mobile": {},
            "firstName": {},
            "middleName": {},
            "lastName": {}
        }}
    "#, user.email.unwrap_or_default(), user.mobile.unwrap_or_default(), user.first_name.unwrap_or_default(), user.middle_name.unwrap_or_default(), user.last_name.unwrap_or_default())).as_object().unwrap()
    )
    .header(ACCEPT, "application/json");

    let requ = SXL::UserRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: usrcpy.get_data().clone()
    };

    let resp = req.send().unwrap();
    let uresp = SXL::UserResponse::new(resp);
    Log {
        req_t: SXL::RequestType::User,
        request: Box::new(requ),
        response: Box::new(uresp),
    }
    

}
