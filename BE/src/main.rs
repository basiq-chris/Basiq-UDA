
use BSAPI::{requestHandler::send_request, Token};
use actix_web::{Responder, HttpServer, App, HttpResponseBuilder, web};
use qstring::QString;
use reqwest::{StatusCode, ResponseBuilderExt, RequestBuilder, header::ACCEPT, Client, Method};
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
        .service(create_auth_link)
        .service(get_job)
        .app_data(token.clone())
    })
    .bind(("127.0.0.1", 8642))?
    .run()
    .await
}

struct ServerToken {
    token: Mutex<BSAPI::Token>
}

#[actix_web::get("/token/{user_id}")]
async fn get_client_token(request_body: web::Path<String>, server_token: web::Data<ServerToken>) -> impl Responder {
    println!("INFO: GET request made to /token");
    let user_id = request_body.into_inner();
    println!("DEBUG: userID used: {}", user_id);
    HttpResponseBuilder::new(StatusCode::CREATED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(reqwest::Client::new(), BSAPI::RequestType::Token(BSAPI::KeyType::CLIENT_ACCESS), reqwest::Method::POST, None, None).await.stringify())
}

#[actix_web::post("/createuser")]
async fn create_user(response_body: String, server_token: actix_web::web::Data<ServerToken>) -> impl Responder {
    let query = QString::from(response_body.as_str());
    println!("DEBUG: Body Content: {:?}", query);
    println!("DEBUG: Checking token health");
    let mut token = server_token.token.lock().unwrap();
    if token.has_expired() {
        println!("INFO: Token Expired");
        *token = get_server_token().await;
    }
    let tkn = token.clone();
    drop(token);
    println!("INFO: POST request made to /createuser");
    HttpResponseBuilder::new(StatusCode::CREATED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(Client::new(), BSAPI::RequestType::Users(vec![query.get("email").unwrap_or_else(|| "").to_string(), query.get("mobile").unwrap_or_else(|| "").to_string(), query.get("firstName").unwrap_or_else(|| "").to_string(), query.get("middleName").unwrap_or_else(|| "").to_string(), query.get("lastName").unwrap_or_else(|| "").to_string()]), reqwest::Method::POST, Some(tkn), None).await.stringify())
}

#[actix_web::post("/createauthlink")]
async fn create_auth_link(response_body: String, server_token: actix_web::web::Data<ServerToken>) -> impl Responder {
    let query = QString::from(response_body.as_str());
    println!("DEBUG: Body Content: {:?}", query);
    println!("DEBUG: Checking token health");
    let mut token = server_token.token.lock().unwrap();
    if token.has_expired() {
        println!("INFO: Token Expired");
        *token = get_server_token().await;
    }
    let tkn = token.clone();
    drop(token);
    println!("INFO: POST request made to /createauthlink");
    HttpResponseBuilder::new(StatusCode::CREATED)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(Client::new(), BSAPI::RequestType::AuthLink(query.get("userID").map(String::from).unwrap()), Method::POST, Some(tkn), None).await.stringify())
}

#[actix_web::get("/getjob/{job_id}")]
async fn get_job(url_params: web::Path<String>, server_token: actix_web::web::Data<ServerToken>) -> impl Responder {
    let query = url_params.into_inner();
    println!("DEBUG: Body Content: {:?}", query);
    println!("DEBUG: Checking token health");
    let mut token = server_token.token.lock().unwrap();
    if token.has_expired() {
        println!("INFO: Token Expired");
        *token = get_server_token().await;
    }
    let tkn = token.clone();
    drop(token);
    println!("INFO: GET request made to /getjob");
    HttpResponseBuilder::new(StatusCode::OK)
    .append_header(("Access-Control-Allow-Origin", "*"))
    .append_header(("Access-Control-Allow-Methods", "GET,POST,DELETE"))
    .append_header(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
    .body(BSAPI::requestHandler::send_request(Client::new(), BSAPI::RequestType::Jobs(query), Method::GET, Some(tkn), None).await.stringify())
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


