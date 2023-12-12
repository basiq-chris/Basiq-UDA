use SXL::{RequestLog, ResponseLog, Log};
use serde_json::Value;
use reqwest::{self, Method, Request, header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION}};
use crate as BSAPI;

fn send_request(client: reqwest::blocking::Client, request_type: BSAPI::RequestType, method: reqwest::Method, token: Option<String>, data: Option<String>) -> SXL::Log {
    let urlbase = "https://au-api.basiq.io/";
    match request_type {
        BSAPI::RequestType::Token(val, typ) => {
            match method {
                 reqwest::Method::POST => {
                    match typ {
                        BSAPI::KeyType::SERVER_ACCESS => {
                    let req = client.post(urlbase.to_owned() + "/token")
                    .header(ACCEPT, "application/json")
                    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(AUTHORIZATION, "Basic ".to_owned() + val.as_str());
                    
                    let reql = RequestLog::new(&req, val);
                    let resp = req.send().unwrap();
                    let resl = ResponseLog::new(resp);

                    return SXL::Log {
                        req: reql,
                        res: resl
                    }
                 },
                 BSAPI::KeyType::CLIENT_ACCESS => {
                     let req = client.post(urlbase.to_owned() + "/token")
                     .header(ACCEPT, "application/json")
                     .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                     .header(AUTHORIZATION, "Basic ".to_owned() + val.as_str())
                     .query(&[("userId", data.unwrap()), ("scope", "CLIENT_ACCESS".to_string())]);
                    
                     
                     let reql = RequestLog::new(&req, val);
                     let resp = req.send().unwrap();
                     let resl = ResponseLog::new(resp);
 
                     return SXL::Log {
                         req: reql,
                         res: resl
                     }
                }
                }
                }
                 _ => panic!("Basiq API does not allow {} requests for this endpoint", method.as_str())
            }
        }
        BSAPI::RequestType::Users(val) => {
            match method {
                reqwest::Method::POST => {
                    if val[0].len() == 36 {
                        let unf_json = format!(r#"
                        {{
                            "email": {},
                            "mobile": {},
                            "firstName": {},
                            "middleName": {},
                            "lastName": {}
                        }}
                        "#, val[1].as_str(), val[2].as_str(), val[3].as_str(), val[4], val[5]);

                        let req = client.post(urlbase.to_owned() + "/users/" + val[0].as_str())
                        .bearer_auth(token.unwrap())
                        .header(ACCEPT, "application/json")
                        .header(CONTENT_TYPE, "application/json")
                        .json(serde_json::from_str::<&str>(unf_json.clone().as_str()).unwrap());

                        let reql = RequestLog::new(&req, unf_json.to_string());
                        let resp = req.send();
                        let resl = ResponseLog::new(resp.unwrap());
                        Log {
                            req: reql,
                            res: resl
                        }
                    }
                    else {
                        let unf_json = format!(r#"
                        {{
                            "email": {},
                            "mobile": {},
                            "firstName": {},
                            "middleName": {},
                            "lastName": {}
                        }}
                        "#, val[0].as_str(), val[1].as_str(), val[2].as_str(), val[3], val[4]);

                        let req = client.post(urlbase.to_owned() + "/users/" + val[0].as_str())
                        .bearer_auth(token.unwrap())
                        .header(ACCEPT, "application/json")
                        .header(CONTENT_TYPE, "application/json")
                        .json(serde_json::from_str::<&str>(unf_json.clone().as_str()).unwrap());

                        let reql = RequestLog::new(&req, unf_json.to_string());
                        let resp = req.send();
                        let resl = ResponseLog::new(resp.unwrap());
                        Log {
                            req: reql,
                            res: resl
                        }
                    }
                }
                _ => panic!("Unsupported")
            }
        },
        BSAPI::RequestType::Consent(_) => todo!(),
        BSAPI::RequestType::AuthLink(_) => todo!(),
        BSAPI::RequestType::Jobs(_) => todo!(),
    }
}