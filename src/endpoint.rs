use std::{time::Instant};

use actix_web::{get, web::Data, HttpResponse};
use awc::{Client};

// CONSTS
const MAP_URL: &str =
    "https://code-challenge-a.wellcome-data.org/api";

// GET - WELLCOME API ENDPOINT
#[get("/")]
pub async fn fetch_data(client: Data<Client>) -> HttpResponse {
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