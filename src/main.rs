use std::{sync::Arc};

use actix_web::{middleware, web::Data, App, HttpServer};
use awc::{http::header, Client, Connector};

mod config;
mod endpoint;

// CONSTS
const TOKEN: &str =
    "Bearer aGV5bm9sb29raW5naW5oZXJldGhpc2lzc2VjcmV0c2hoaGg="; // should be in .env file

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
            .service(endpoint::fetch_data)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
