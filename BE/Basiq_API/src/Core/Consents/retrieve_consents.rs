use SXL::{ConsentRequest, SXLoggableRequest, ConsentResponse, Consent, Log};
use reqwest::header::ACCEPT;


pub fn get(thread_client: reqwest::blocking::Client, token: String, user_id: String) -> Log {
    let req = thread_client.get("https://au-api.basiq.io/users/".to_owned() + &user_id + "/consents")
    .bearer_auth(token)
    .header(ACCEPT, "application/json");

    let creq = ConsentRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: user_id,
    };

    let resp = creq.send();

    let cresp = ConsentResponse {
        status: resp.status(),
        header: resp.headers().clone(),
        data: Consent {
            data: serde_json::from_str(resp.text().unwrap().as_str()).unwrap()
        },
    };

    Log {
        req_t: SXL::RequestType::Consent,
        request: Box::new(creq),
        response: Box::new(cresp)
    }
}