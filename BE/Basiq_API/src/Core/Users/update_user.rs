use reqwest::header::ACCEPT;


pub fn update(user_id: String, user_info: SXL::User, tkn: String, thread_client: reqwest::blocking::Client) -> SXL::Log {
    let usrcpy = user_info.clone();
    let req = thread_client.post("https://au-api.basiq.io/users/".to_owned() + user_id.as_str())
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
    "#, user_info.email.unwrap_or_default(), user_info.mobile.unwrap_or_default(), user_info.first_name.unwrap_or_default(), user_info.middle_name.unwrap_or_default(), user_info.last_name.unwrap_or_default())).as_object().unwrap()
    )
    .header(ACCEPT, "application/json");

    let ureq = SXL::UserRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: String::from(format!(r#"
        {{
            "user_id": "{}",
            "user_data": "{}"
        }}
        "#, user_id, usrcpy.get_data())),
    };

    let resp = req.send().unwrap();

    let uresp = SXL::UserResponse {
        status: resp.status(),
        headers: resp.headers().clone(),
        data: serde_json::from_str(resp.text().unwrap().as_str()).unwrap(),
    };

    SXL::Log {
        req_t: SXL::RequestType::User,
        request: Box::new(ureq),
        response: Box::new(uresp),
    }
}