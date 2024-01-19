use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::HeaderName;
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
        Logger::print_debug(json.clone());
        let request_type = json["type"].as_str();
        match request_type {
            Some(value) => {
                let mut data: Vec<Box<(String, String)>> = Vec::new();
                match value {
                    "user" => {
                        if json["firstName"].as_str().is_none() {
                            data.push(Box::new(("userID".to_string(), json["id"].as_str().unwrap().to_string())));
                            base.data = data;
                        } else {
                            for x in ["id", "email", "mobile", "firstName", "middleName", "lastName"].iter() {
                                base.data.push(Box::new((x.to_string(), json[x].as_str().unwrap_or_default().to_string())));
                            }
                        }
                    }
                    "auth_link" => {
                        data.push(Box::new(("userID".to_string(), json["userId"].as_str().unwrap().to_string())));
                        data.push(Box::new(("mobile".to_string(), json["mobile"].as_str().unwrap_or_else(|| "no-mobile").to_string())));
                        data.push(Box::new(("authLink".to_string(), json["links"]["public"].as_str().unwrap().to_string())));
                        base.data = data;
                    }
                    "job" => {
                        data.push(Box::new(("jobID".to_string(), json["id"].as_str().unwrap().to_string())));
                        Logger::print_debug(json["created"].as_str().unwrap());
                        let mut stepinfo: Vec<Box<(String, String)>> = Vec::new();
                        for step in json["steps"].as_array().unwrap() {
                            stepinfo.push(Box::new((step["title"].as_str().unwrap().to_string(), step["status"].as_str().unwrap().to_string())));
                        }
                        data.append(&mut stepinfo);
                        let mut extradata: Vec<Box<(String, String)>> = Vec::new();
                        let publink = json["links"]["source"].as_str().unwrap().to_string();
                        let publink: Vec<String> = publink.split("/").map(|x| x.to_string()).collect();
                        extradata.push(Box::new(("userID".to_string(), publink[4].clone())));
                        extradata.push(Box::new(("connectionID".to_string(), publink[6].clone())));
                        drop(publink);
                        data.append(&mut extradata);
                        base.data = data; 
                    }
                    "list" => base.data = Self::list_handler(json["data"].as_array().unwrap()),
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

        fn list_handler(list_json: &Vec<Value>) -> Vec<Box<(String, String)>> {
            let mut data: Vec<serde_json::Map<String, Value>> = Vec::new();
            let ttype: String = list_json[0]["type"].as_str().unwrap_or_else(|| "null").to_string();

            for e in list_json {
                match e["type"].as_str().unwrap() {
                    "account" => {
                        let mut account: serde_json::Map<String, Value> = serde_json::Map::new();
                        for key in ["id", "accountHolder", "accountNo", "availableFunds", "balance", "name", "institution"] {
                            account.insert(key.to_string(), e[key].clone());
                        }
                        data.push(account);
                    }
                    "transaction" => {
                        let mut transaction = serde_json::Map::new();
                        for key in ["postDate", "description", "amount"] {
                            transaction.insert(key.to_string(), e[key].clone());
                        }
                        data.push(transaction);
                    }
                    "error" => panic!("Error recieved from Basiq\ncode: {}\ntitle: {}\ndetail: {}", e["code"].as_str().unwrap(), e["title"].as_str().unwrap(), e["detail"].as_str().unwrap()),
                    _ => panic!("Unknown list item recieved: '{}'", e["type"].as_str().unwrap())
                }
            }
            let mut json_array: Vec<Value> = Vec::new(); 
            for ele in data {
                json_array.push(Value::Object(ele));
            }

            return vec![Box::new((ttype, Value::Array(json_array).to_string()))];
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
            if value.0.contains(|x| x == '[') || value.1.contains(|x| x == '[') {
                payload += &format!(r#""{}":{},"#, value.0.as_str(), value.1.as_str());
            } else {
            payload += &format!(r#""{}":"{}","#, value.0.as_str(), value.1.as_str());
            }
        }
        payload.pop();
        let response = format!(r#"{{"headers":{{{}}},"status":"{}", "payload":{{{}}}}}"#, response_headers, self.res.status.as_str(), payload);
        String::from(format!(r#"{{"request_data":{},"response_data":{}}}"#, request, response))
    }
}