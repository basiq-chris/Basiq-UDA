use std::{ops::Deref, panic, time::{SystemTime, UNIX_EPOCH}, fmt::format};

use reqwest::{Response, header::HeaderName};
use serde_json::Value;


pub struct RequestLog {
    pub header: reqwest::header::HeaderMap,
    pub verb: reqwest::Method,
    pub data: Vec<Box<(String, String)>>
}

impl RequestLog {
    pub fn new(rb: &reqwest::RequestBuilder, data: Vec<Box<(String, String)>>) -> Self {
        let rbb = rb.try_clone().unwrap().build().unwrap();
        return RequestLog { header: rbb.headers().clone(), verb: rbb.method().clone(), data}
    }
}

pub struct ResponseLog {
    pub header: reqwest::header::HeaderMap,
    pub status: reqwest::StatusCode,
    pub data: Vec<Box<(String, String)>>
}

impl ResponseLog {
    pub async fn new(res: reqwest::Response) -> Self {
       let mut base = ResponseLog {
            header: res.headers().clone(),
            status: res.status(),
            data: Vec::new()
        };
        let json: Value = res.json().await.unwrap();
        let request_type = json["type"].as_str();
        match request_type {
            Some(value) => {
                match value {
                    "user" => {
                        if json["firstName"].as_str().is_none() {
                            let mut data: Vec<Box<(String, String)>> = Vec::new();
                            data.push(Box::new(("userID".to_string(), json["id"].as_str().unwrap().to_string())));
                            base.data = data;
                        } else {
                            for x in ["id", "email", "mobile", "firstName", "middleName", "lastName"].iter() {
                                base.data.push(Box::new((x.to_string(), json[x].as_str().unwrap_or_default().to_string())));
                            }
                        }
                    }
                    
                    _ => panic!("Unsupported type {}; Mention this to the package maintainer", value)
                }
            },
            None => {
                match json["access_token"].as_str().is_some() {
                    true => {base.data.push(Box::new(("expiry".to_string(), (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + json["expires_in"].as_u64().unwrap()).to_string())));
                                base.data.push(Box::new(("access_token".to_string(), json["access_token"].as_str().unwrap().to_string())))}
                    false => {panic!("Not supported or illegal")}
                }
            }
        }

        base
        }
    }

pub struct Log {
    pub req: RequestLog,
    pub res: ResponseLog
}

impl Log {
    pub fn stringify(self) -> String {
        let mut request_headers: String = String::from("");
        for x in self.req.header {
            request_headers += &("\"".to_owned() + x.0.unwrap_or_else(|| HeaderName::from_static("Null-Header")).as_str() + "\"" + ": " + "\"" + x.1.to_str().unwrap() + "\"" + ",");
        }
        request_headers.pop();
        let mut payload: String = String::from("");
        for x in self.req.data {
            let value = *x;
            payload += &format!(r#""{}":"{}","#, value.0.as_str(), value.1.as_str());
        }
        payload.pop();
        let request = format!(r#"{{"headers":{{{}}},"request-verb":"{}", "payload":{{{}}}}}"#, request_headers, self.req.verb, payload);
        let mut response_headers: String = String::from("");
        for x in self.res.header {
            response_headers += &("\"".to_owned() + x.0.unwrap_or_else(|| HeaderName::from_static("Null-Header")).as_str() + "\"" + ": " + "\"" + x.1.to_str().unwrap() + "\"" + ",");
        }
        response_headers.pop();
        let mut payload: String = String::from("");
        for x in self.res.data {
            let value = *x;
            payload += &format!(r#""{}":"{}","#, value.0.as_str(), value.1.as_str());
        }
        payload.pop();
        let response = format!(r#"{{"headers":{{{}}},"status":"{}", "payload":{{{}}}}}"#, response_headers, self.res.status.as_str(), payload);
        String::from(format!(r#"{{"request_data":{},"response_data":{}}}"#, request, response))
    }
}