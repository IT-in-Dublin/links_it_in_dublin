use actix_web::{ App, HttpServer};
mod read_static;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(read_static::main)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}