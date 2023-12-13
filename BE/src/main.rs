use std::future::Future;

use BSAPI::{requestHandler::send_request, Token};
use actix_web::{Responder, HttpServer, App, HttpResponseBuilder};
use reqwest::{StatusCode, ResponseBuilderExt, RequestBuilder, header::ACCEPT};
use Basiq_API as BSAPI;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("DEBUG: Server created");
    HttpServer::new(|| {
        App::new()
        .service(get_client_token)
        .service(create_user)
        .app_data(actix_web::web::Data::new(ServerState {
            token: Mutex::new(None),
            users: Mutex::new(None)
        }))
    })
    .bind(("127.0.0.1", 8642))?
    .run()
    .await
}

struct ServerState {
    pub token: Mutex<Option<BSAPI::Token>>,
    pub users: Mutex<Option<Vec<String>>>
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
async fn create_user(response_body: String, data: actix_web::web::Data<ServerState>) -> impl Responder {
    println!("DEBUG: Body Content: {}", response_body);
    println!("DEBUG: Checking token health");
    check_token(&data.token).await;
    println!("INFO: POST request made to /createuser");
    HttpResponseBuilder::new(StatusCode::NOT_IMPLEMENTED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body("Function Currently not Implemented")
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

async fn check_token(token: &Mutex<Option<BSAPI::Token>>) {
   let mut token_info = token.lock().unwrap();
   if token_info.is_none() || token_info.as_ref().is_some_and(|x| x.has_expired()) {
        *token_info = Some(get_server_token().await)
   }
}

