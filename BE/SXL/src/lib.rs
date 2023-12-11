use std::{ops::Deref, panic};

use reqwest::Response;
use serde_json::Value;


pub struct RequestLog {
    pub header: reqwest::header::HeaderMap,
    pub verb: reqwest::Method,
    pub data: Box<String>
}

impl RequestLog {
    pub fn new(rb: &reqwest::blocking::RequestBuilder, data: String) -> Self {
        let rbb = rb.try_clone().unwrap().build().unwrap();
        return RequestLog { header: rbb.headers().clone(), verb: rbb.method().clone(), data: Box::new(data) }
    }
}

pub struct ResponseLog {
    pub header: reqwest::header::HeaderMap,
    pub status: reqwest::StatusCode,
    pub data: Box<String>
}

impl ResponseLog {
    pub fn new(res: reqwest::blocking::Response) -> Self {
       let mut base = ResponseLog {
            header: res.headers().clone(),
            status: res.status(),
            data: Box::new(String::from(""))
        };

        let json: Value = res.json().unwrap();
        let request_type = json["type"].as_str();
        match request_type {
            Some(value) => {
                match value {
                    "user" => {
                        if json["firstName"].as_str().is_none() {
                            base.data = Box::new(json["id"].as_str().map(String::from).unwrap());
                        } else {
                            base.data = Box::new(format!(r#"{{"id":{},"email":{},"mobile":{},"firstName":{},"middleName":{},"lastName":{}}}"#, json["id"].as_str().unwrap_or_default(), json["email"].as_str().unwrap_or_default(), json["mobile"].as_str().unwrap_or_default(), json["firstName"].as_str().unwrap_or_default(), json["middleName"].as_str().unwrap_or_default(), json["lastName"].as_str().unwrap_or_default()));
                        }
                    }
                    _ => panic!("Unsupported type {}; Mention this to the package maintainer", value)
                }
            },
            None => {
                match json["access_token"].as_str() {
                    Some(val) => {base.data = Box::new(val.to_string())}
                    None => {panic!("Not supported or illegal")}
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