use SXL::{JobRequest, JobsResponse, SXLoggableRequest, Job, iJob, Log};
use reqwest::header::ACCEPT;

pub fn get(thread_client: reqwest::blocking::Client, token: String, job_id: String) -> Log {
    let req = thread_client.get("https://au-api.basiq.io/jobs/".to_owned() + &job_id)
    .header(ACCEPT, "application/json")
    .bearer_auth(token);

    let jreq = JobRequest {
        request_data: req.try_clone().unwrap(),
        verb: req.try_clone().unwrap().build().unwrap().method().clone(),
        header: req.try_clone().unwrap().build().unwrap().headers().clone(),
        data: job_id,
    };

    let resp = jreq.send();

    let jresp = JobsResponse {
        status: resp.status(),
        header: resp.headers().clone(),
        data: vec![iJob {
            institution: "AU00000".to_string(),
            jobs: Job::from_job_object(serde_json::from_str(resp.text().unwrap().as_str()).unwrap())
        }]
    };

    Log {
        req_t: SXL::RequestType::Job,
        request: Box::new(jreq),
        response: Box::new(jresp),
    }
}