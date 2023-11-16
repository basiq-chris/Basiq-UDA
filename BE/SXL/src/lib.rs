
pub enum RequestType {
    User,
    Token,
}

pub enum Direction {
    Request,
    Response,
}

pub struct Token {
    pub token: String,
    pub expires: u64
}

pub trait SXLoggableRequest {
    fn get_verb(&self) -> reqwest::Method;
    fn get_headers(&self) -> reqwest::header::HeaderMap;
    fn get_data(&self) -> String;
    fn send(&self) -> reqwest::blocking::Response;
    }

pub trait SXLoggableResponse {
    fn get_status(&self) -> reqwest::StatusCode;
    fn get_headers(&self) -> reqwest::header::HeaderMap;
    fn get_data(&self) -> String;
}

pub struct Log<Req: SXLoggableRequest, Res: SXLoggableResponse> {
    pub req_t: RequestType,
    pub request: Req,
    pub response: Res,
}

pub struct TokenRequest {
    request_data: reqwest::blocking::RequestBuilder,
    api_key: String,
}

impl SXLoggableRequest for TokenRequest {
    fn get_data(&self) -> String {
        self.api_key.clone()
    }
    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.request_data.headers().clone()
    }
    fn get_verb(&self) -> reqwest::Method {
        self.request_data.method().clone()
    }
    fn send(&self) -> reqwest::blocking::Response {
        self.request_data.send
    }
}

pub struct TokenResponse {
    pub response_data: reqwest::blocking::Response,
    pub token: Token
}

impl SXLoggableResponse for TokenResponse {
    fn get_status(&self) -> reqwest::StatusCode {
        self.response_data.status().clone()
    }
    fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.response_data.headers().clone()
    }
    fn get_data(&self) -> String {
        self.token.token.clone()
    }
}

#[derive(Default)]
pub struct LogQueue<Req: SXLoggableRequest, Res: SXLoggableResponse> {
    pub queue: std::collections::VecDeque<Log<Req, Res>>,
}

impl<Req: SXLoggableRequest, Res: SXLoggableResponse> LogQueue<Req, Res> {
    pub fn new() -> Self {
        LogQueue {
            queue: std::collections::VecDeque::new(),
        }
    }

    pub fn push(log: Log<Req, Res>, q: &mut Self) {
        q.queue.push_back(log)
    }

    pub fn pull(q: &mut Self) -> Result<Log<Req, Res>, u8> {
        match q.queue.pop_front() {
            Some(val) => Ok(val),
            None => Err(0),
        }
    }
}
