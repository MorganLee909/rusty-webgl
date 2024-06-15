mod frontend;

use actix_web::{App, HttpServer, web};
use crate::frontend::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("")
                .route("/", web::get().to(html))
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
