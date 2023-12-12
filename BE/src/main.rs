use actix_web::{HttpResponse, Responder, HttpServer, App};



#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
        .service(get_client_token)
    })
    .bind(("127.0.0.1", 8642))?
    .run()
    .await
}

#[actix_web::get("/token")]
pub async fn get_client_token() -> impl Responder {
    HttpResponse::Created().body("Hello World!")
}

