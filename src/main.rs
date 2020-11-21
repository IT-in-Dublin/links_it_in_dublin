use actix_web::{ App, HttpServer};
mod read_me;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(read_me::handle_content)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}