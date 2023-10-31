use actix_web::{App, HttpResponse, HttpServer, middleware::DefaultHeaders, get};
use chrono::Local;

#[get("/api/time")]
async fn index() -> HttpResponse {
    let now = Local::now();
    let response = format!("The time now is {}", now);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "http://localhost:3000")) // Adjust the origin to match your Nefertari URL
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
