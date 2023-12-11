pub mod requestHandler;

pub enum RequestType {
    Token(String),
    Users(Vec<String>),
    Consent(String),
    AuthLink(String),
    Jobs(String)
}