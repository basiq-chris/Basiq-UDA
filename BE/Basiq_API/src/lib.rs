pub mod requestHandler;

pub enum RequestType {
    //API KEY
    Token(String),
    //ID/email, mobile, fname, mname, lname
    Users(Vec<String>),
    //ID
    Consent(String),
    //ID/userID
    AuthLink(String),
    //ID/userID
    Jobs(String)
}