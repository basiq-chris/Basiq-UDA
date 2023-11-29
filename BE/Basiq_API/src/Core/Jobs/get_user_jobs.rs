use SXL::{JobRequest, SXLoggableRequest, JobsResponse, Log};
use reqwest::header::ACCEPT;

pub fn get(thread_client: reqwest::blocking::Client, token: String, user_id: String) -> Log {
    let req = thread_client.get("https://au-api.basiq.io/users/".to_owned() + &user_id + "/jobs")
    .header(ACCEPT, "application/json")
    .bearer_auth(token);

    let jreq = JobRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        header: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: user_id,
    };

    let resp = jreq.send();

    let jresp = JobsResponse {
        status: resp.status(),
        header: resp.headers().clone(),
        data: JobsResponse::add_jobs(serde_json::from_str(resp.text().unwrap().as_str()).unwrap()),
    };

    Log {
        req_t: SXL::RequestType::Job,
        request: Box::new(jreq),
        response: Box::new(jresp)
    }
}