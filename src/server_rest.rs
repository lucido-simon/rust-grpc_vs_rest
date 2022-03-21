use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
pub async fn hello() -> impl Responder {
    business_logic_hello()
}

pub fn business_logic_hello() -> impl Responder {
    HttpResponse::Ok().body("Testing !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP server on port {}", 8080);
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
