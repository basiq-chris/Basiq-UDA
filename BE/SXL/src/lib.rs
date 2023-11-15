
pub enum RequestType {
    User,
    Token
}

pub enum Direction {
    Request,
    Response
}

pub trait SXLoggableRequest {
    
}

pub trait SXLoggableResponse {
    
}


pub struct Log<Req: SXLoggableRequest, Res: SXLoggableResponse> {
    pub req_t: RequestType,
    pub request: Req,
    pub response: Res
}

pub struct TokenLog {
    
}


#[derive(Default)]
pub struct LogQueue<Req: SXLoggableRequest, Res: SXLoggableResponse> {
    pub queue: std::collections::VecDeque<Log<Req, Res>>
}

impl<Req: SXLoggableRequest, Res: SXLoggableResponse> LogQueue<Req, Res> {
    pub fn new() -> Self {
        LogQueue {
            queue: std::collections::VecDeque::new()
        }
    }

    pub fn push(log: Log<Req, Res>, q: &mut Self) {
        q.queue.push_back(log)
    }

    pub fn pull(q: &mut Self) -> Result<Log<Req, Res>, u8> {
        match q.queue.pop_front() {
            Some(val) => Ok(val),
            None => {Err(0)}
        }
    }
}
