
pub fn delete(user_id: String, token: String, thread_client: reqwest::blocking::Client) -> SXL::Log {
    let req = thread_client
    .delete("https://au-api.basiq.io/users/".to_owned() + user_id.as_str())
    .bearer_auth(token)
    .header(reqwest::header::ACCEPT, "application/json");

    let cloned_req = req.try_clone().unwrap();
    let ureq = SXL::UserRequest {
        request_data: cloned_req.try_clone().unwrap(),
        verb: cloned_req.try_clone().unwrap().build().unwrap().method().clone(),
        headers: cloned_req.build().unwrap().headers().clone(),
        data: user_id
    };
    let resp = req.send().unwrap();
    let uresp = SXL::UserResponse::new(resp);

    SXL::Log {
        req_t: SXL::RequestType::User,
        request: Box::new(ureq),
        response: Box::new(uresp)
    }

}