use SXL::{Log, UserResponse};
use reqwest::header::ACCEPT;





pub fn retrieve_user(user_id: String, Token: String, thread_client: reqwest::blocking::Client) -> Log {
    let req = thread_client.get("https://au-api.basiq.io/users/".to_owned() + user_id.as_str())
    .header(ACCEPT, "application/json")
    .bearer_auth(Token);

    let ureq = SXL::UserRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: user_id,
    };

    let resp = req.send().unwrap();
    let uresp = UserResponse {
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