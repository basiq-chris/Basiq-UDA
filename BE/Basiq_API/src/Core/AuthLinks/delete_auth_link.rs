use SXL::{Log, AuthLinkRequest, AuthLink, SXLoggableRequest};
use reqwest::header::ACCEPT;

pub fn delete(thread_client: reqwest::blocking::Client, token: String, user_id: String) -> Log {
    let req = thread_client.delete("https://au-api.basiq.io/users/".to_owned() + &user_id + "/auth_link")
    .header(ACCEPT, "application/json")
    .bearer_auth(token);

    let alreq = AuthLinkRequest {
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

    Log {
        req_t: SXL::RequestType::AuthLink,
        request: Box::new(alreq),
        response: Box::new(alresp),
    }
}