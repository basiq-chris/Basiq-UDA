use std::{io::Read, str::FromStr};

use SXL::{RequestLog, ResponseLog, Log};
use serde_json::Value;
use reqwest::{self, Method, Request, header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION}, blocking::RequestBuilder};
use crate as BSAPI;

pub async fn send_request(client: reqwest::Client, request_type: BSAPI::RequestType, method: reqwest::Method, token: Option<String>, data: Option<String>) -> SXL::Log {
    let urlbase = "https://au-api.basiq.io";
    match request_type {
        BSAPI::RequestType::Token(typ) => {
            match method {
                 reqwest::Method::POST => {
                    let mut val: String = String::from("");
                    let mut reader = std::fs::File::open("./Basiq_API/APIKEY.env").unwrap();
                    let _ = reader.read_to_string(&mut val);
                    println!("DEBUG: API KEY GOT: {}", val);
                    match typ {
                        BSAPI::KeyType::SERVER_ACCESS => {
                    let req = client.post(urlbase.to_owned() + "/token")
                    .header(ACCEPT, "application/json")
                    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header("basiq-version", "3.0")
                    .header(AUTHORIZATION, "Basic ".to_owned() + val.as_str());
                    let reql = RequestLog::new(&req, vec![Box::new(("API_KEY".to_string(), val.to_string()))]);
                    let resp = req.send().await.unwrap();
                    let resl = ResponseLog::new(resp);

                    return SXL::Log {
                        req: reql,
                        res: resl.await
                    };
                 },
                 BSAPI::KeyType::CLIENT_ACCESS => {
                     let req = client.post(urlbase.to_owned() + "/token")
                     .header(ACCEPT, "application/json")
                     .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                     .header(AUTHORIZATION, "Basic ".to_owned() + val.as_str())
                     .query(&[("userId", data.unwrap()), ("scope", "CLIENT_ACCESS".to_string())]);
                    
                     
                     let reql = RequestLog::new(&req, vec![Box::new(("API_KEY".to_string(), val.to_string()))]);
                     let resp = req.send().await.unwrap();
                     let resl = ResponseLog::new(resp);
    
                     return SXL::Log {
                         req: reql,
                         res: resl.await
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
                            "email": "{}",
                            "mobile": "{}",
                            "firstName": "{}",
                            "middleName": "{}",
                            "lastName": "{}"
                        }}
                        "#, val[1].as_str(), val[2].as_str(), val[3].as_str(), val[4], val[5]);

                        let req = client.post(urlbase.to_owned() + "/users/" + val[0].as_str())
                        .bearer_auth(token.unwrap())
                        .header(ACCEPT, "application/json")
                        .header(CONTENT_TYPE, "application/json")
                        .json(serde_json::from_str::<&str>(unf_json.clone().as_str()).unwrap());

                        let data:Vec<Box<(String, String)>>;
                        {
                            let formatted = Value::from_str(unf_json.as_str()).unwrap().as_object().unwrap().clone();
                            let mut arr: Vec<Box<(String, String)>> = Vec::new();
                            for x in formatted {
                                arr.push(Box::new((x.0, x.1.to_string())));
                            }
                            data = arr;
                        };
                        let reql = RequestLog::new(&req, data);
                    
                        let resp = req.send().await;
                        let resl = ResponseLog::new(resp.unwrap());
                        return Log {
                            req: reql,
                            res: resl.await
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


                        let data:Vec<Box<(String, String)>>;
                        {
                            let formatted = Value::from_str(unf_json.as_str()).unwrap().as_object().unwrap().clone();
                            let mut arr: Vec<Box<(String, String)>> = Vec::new();
                            for x in formatted {
                                arr.push(Box::new((x.0, x.1.to_string())));
                            }
                            data = arr;
                        };
                        let reql = RequestLog::new(&req, data);
                        let resp = req.send().await;
                        let resl = ResponseLog::new(resp.unwrap());
                       return Log {
                            req: reql,
                            res: resl.await
                        };
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