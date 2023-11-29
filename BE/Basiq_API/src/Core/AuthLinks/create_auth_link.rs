use SXL::{Log, SXLoggableRequest, AuthLink};
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn post(thread_client: reqwest::blocking::Client, token: String, user_id: String) -> Log {
    let req = thread_client.post("https://au-api.basiq.io/users/".to_owned() + &user_id + "/auth_link")
    .header(ACCEPT, "application/json")
    .header(CONTENT_TYPE, "application/json")
    .bearer_auth(token);


    let alreq = SXL::AuthLinkRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: user_id
    };

    let resp = alreq.send();
    let alresp = SXL::AuthLinkResponse {
        status: resp.status(),
        headers: resp.headers().clone(),
        data: AuthLink {
            data: serde_json::from_str(resp.text().unwrap().as_str()).unwrap()
        },
    };

    SXL::Log {
        req_t: SXL::RequestType::Consent,
        request: Box::new(alreq),
        response: Box::new(alresp),
    }
}