use actix_web::{web, App, HttpServer};

use crate::routes::atmosphere::get_atmosphere;
use crate::routes::index::index;
use crate::AccessSharedData;

pub async fn run_app(sd: &AccessSharedData) -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");
    let common_data = web::Data::new(sd.clone());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .route("/get_atmosphere", web::get().to(get_atmosphere))
            .service(web::resource("/").to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run();

    server.await
}
