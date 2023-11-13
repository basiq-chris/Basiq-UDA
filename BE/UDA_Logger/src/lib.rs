use std::{time::{SystemTime, UNIX_EPOCH}, thread::AccessError, ops::Deref};


pub enum RequestTypes {
    POST,
    GET,
    DELETE
}

pub struct Log {
    pub verb: RequestTypes,
    pub headers: reqwest::header::HeaderMap,
    pub body: reqwest::Body,
    pub timestamp: u64
}

impl Log {
    pub fn create(verb: RequestTypes, headers: reqwest::header::HeaderMap, body: reqwest::Body, timestamp: u64) -> Self {
        Log {
            verb,
            headers,
            body,
            timestamp
        }
    }

    pub fn from_request(req: &reqwest::RequestBuilder) -> Self {
        let built = req.build().unwrap_or_else(|err| panic!("Request cannot be built\n\nError: {}", err));
        Self::create(built.method(), built.headers().clone(), built.body().unwrap(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
}

#[derive(Default)]
pub struct LogQueue {
    pub queue: std::collections::VecDeque<Log>
}

impl LogQueue {
    pub fn new() -> Self {
        LogQueue {
            queue: std::collections::VecDeque::new()
        }
    }

    pub fn push(log: Log, q: &mut Self) {
        q.queue.push_back(log)
    }

    pub fn pull(q: &mut Self) -> Result<Log, u8> {
        match q.queue.pop_front() {
            Some(val) => Ok(val),
            None => {Err(0)}
        }
    }
}
