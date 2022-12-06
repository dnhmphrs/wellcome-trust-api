use std::{str, time::Instant};

use actix_web::{get, web::Data, HttpResponse};
use awc::{Client};

mod sumulative_report;

// CONSTS
const MAP_URL: &str =
    "https://code-challenge-a.wellcome-data.org/api?limit=-1";

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
        // increase payload limit size beyond default
        .limit(100_000_000) // 100MB
        .await
        .expect("Wellcome API Error, Overflow Memory Limit.");

    log::info!(
        "it took {}ms to download api data to memory",
        start.elapsed().as_millis()
    );

    sumulative_report::create_sumulative_report(&payload);

    HttpResponse::Ok()
        .body(payload)
}