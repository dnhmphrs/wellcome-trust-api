use std::{str, time::Instant, time::Duration};

use actix_web::{get, web::Data, HttpResponse};
use awc::{Client};
use serde_json::Value;

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
        .expect("Wellcome API Error");

    log::info!(
        "it took {}ms to download api data to memory",
        start.elapsed().as_millis()
    );

    create_sumulative_report(&payload);

    HttpResponse::Ok()
        .body(payload)
}

// CREATE SUMULATIVE REPORT
fn create_sumulative_report(data: &actix_web::web::Bytes) {
    // log::info!("{}", str::from_utf8(data).unwrap());

    // get json from data
    let json: Value = serde_json::from_slice(data).unwrap();

    // for i in 0..json["items"].as_array().expect("Array Expected").len() {
    //     log::info!("{}", i);
    //     log::info!("{}", json["items"][i]);
    // }

    // The total number of submitted, approved and rejected applications per research area

    // For each of the past 12 months:
    // - the total submitted, approved, and rejected applications in each month

    // - the sum of funding we approved in each month based on the applications data.

    // The average time in (days) between an application being received (submitted) and an outcome (approved or rejected)

    // A list of application ids which have not been actioned in more than 60 days from their submitted date (i.e. they are still in the submitted state).

    // Return bytes object for HTTP Response
}