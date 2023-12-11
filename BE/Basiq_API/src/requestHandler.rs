use SXL::{RequestLog, ResponseLog};
use reqwest::{self, Method, Request, header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION}};
use crate as BSAPI;

pub fn sendRequest(client: reqwest::blocking::Client, request_type: BSAPI::RequestType, method: reqwest::Method) -> SXL::Log {
    let urlbase = "https://au-api.basiq.io/";
    match request_type {
        BSAPI::RequestType::Token(val) => {
            match method {
                 reqwest::Method::POST => {
                    let req = client.post(urlbase.to_owned() + "/token")
                    .header(ACCEPT, "application/json")
                    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(AUTHORIZATION, "Basic ".to_owned() + val.as_str());
                    
                    let reql = RequestLog::new(&req, val);
                    let resp = req.send().unwrap();
                    let resl = ResponseLog::new(resp);

                    SXL::Log {
                        req: reql,
                        res: resl
                    }
                 }
                 _ => panic!("Basiq API does not allow {} requests for this endpoint", method.as_str())
            }
        }
        BSAPI::RequestType::Users(val) => {
            match method {
                reqwest::Method::POST => {
                    if val[0].len() == 36 {
                        client.post(urlbase.to_owned() + "/users/" + val[0].as_str())
                    }
                }
            }
        },
        BSAPI::RequestType::Consent(_) => todo!(),
        BSAPI::RequestType::AuthLink(_) => todo!(),
        BSAPI::RequestType::Jobs(_) => todo!(),
    }
}