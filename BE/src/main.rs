use std::{future::Future, ops::Deref};

use BSAPI::{requestHandler::send_request, Token};
use actix_web::{Responder, HttpServer, App, HttpResponseBuilder};
use qstring::QString;
use reqwest::{StatusCode, ResponseBuilderExt, RequestBuilder, header::ACCEPT, Client};
use Basiq_API as BSAPI;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let token = actix_web::web::Data::new(ServerToken {
        token: Mutex::new(get_server_token().await)
    });
    println!("DEBUG: Server created");
    HttpServer::new(move || {
        App::new()
        .service(get_client_token)
        .service(create_user)
        .app_data(token.clone())
    })
    .bind(("127.0.0.1", 8642))?
    .run()
    .await
}

struct ServerToken {
    token: Mutex<BSAPI::Token>
}

#[actix_web::get("/token")]
async fn get_client_token() -> impl Responder {
    println!("INFO: GET request made to /token");
    HttpResponseBuilder::new(StatusCode::CREATED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(reqwest::Client::new(), BSAPI::RequestType::Token(BSAPI::KeyType::CLIENT_ACCESS), reqwest::Method::POST, None, None).await.stringify())
}

#[actix_web::post("/createuser")]
async fn create_user(response_body: String, server_token: actix_web::web::Data<ServerToken>) -> impl Responder {
    let query = QString::from(response_body.as_str());
    println!("DEBUG: Body Content: {:#?}", query);
    println!("DEBUG: Checking token health");
    let mut token = server_token.token.lock().unwrap();
    if token.has_expired() {
        *token = get_server_token().await;
    }
    println!("INFO: POST request made to /createuser");
    HttpResponseBuilder::new(StatusCode::CREATED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(Client::new(), BSAPI::RequestType::Users(vec![query.get("email").unwrap_or_else(|| " ").to_string(), query.get("mobile").unwrap_or_else(|| " ").to_string(), query.get("firstName").unwrap_or_else(|| " ").to_string(), query.get("middleName").unwrap_or_else(|| " ").to_string(), query.get("lastName").unwrap_or_else(|| " ").to_string()]), reqwest::Method::POST, Some(token.clone()), None).await.stringify())
}

async fn get_server_token() -> Token {
    let req = send_request(reqwest::Client::new(), BSAPI::RequestType::Token(BSAPI::KeyType::SERVER_ACCESS), reqwest::Method::POST, None, None).await;
    Token::new(req.res.data)
}

/* pub async fn get_all_users(token: String) -> Vec<String> {
    let req = reqwest::Client::get("https://au-api.basiq.io/users")
    .header(ACCEPT, "application/json")
    .bearer_auth(token)
    .send().await.unwrap();
} */


