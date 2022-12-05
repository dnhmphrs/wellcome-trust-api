use std::{sync::Arc, time::Instant};

use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer};
use awc::{http::header, Client, Connector};

mod config;

// CONSTS
const MAP_URL: &str =
    "https://code-challenge-a.wellcome-data.org/api";

const TOKEN: &str =
    "Bearer aGV5bm9sb29raW5naW5oZXJldGhpc2lzc2VjcmV0c2hoaGg="; // should be in .env file

// DEFINE ENDPOINT
#[get("/")]
async fn fetch_data(client: Data<Client>) -> HttpResponse {
    let start = Instant::now();

    let mut res = client
        .get(MAP_URL)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        log::error!("Wellcome API did not return expected data");
        return HttpResponse::InternalServerError().finish();
    }

    let payload = res
        .body()
        .await
        .unwrap();

    log::info!(
        "it took {}ms to download api data to memory",
        start.elapsed().as_millis()
    );

    HttpResponse::Ok()
        .body(payload)
}

// DEFINE WEB SERVER && CLIENT
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let client_tls_config = Arc::new(config::client_config());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        // create client _inside_ `HttpServer::new` closure to have one per worker thread
        let client = Client::builder()
            // Wellcome API requires Bearer Authentication to make requests
            .add_default_header((header::AUTHORIZATION, TOKEN))
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls(Arc::clone(&client_tls_config)))
            .finish();

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(client))
            .service(fetch_data)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
