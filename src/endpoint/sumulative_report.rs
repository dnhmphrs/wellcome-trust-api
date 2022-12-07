use serde_json::Value;
use serde_json::Result;
use serde::Serialize;

// CREATE SUMULATIVE REPORT
pub fn create_sumulative_report(data: &actix_web::web::Bytes) -> Result<String> {
  // log::info!("{}", str::from_utf8(data).unwrap());

  // get json from data
  let json: Value = serde_json::from_slice(data).unwrap();

  let mut j = 0;
  for i in 0..json["items"].as_array().expect("Array Expected").len() {
      // log::info!("{}", i);
      // log::info!("{}", json["items"][i]);
      j += 1;
  }

  log::info!("{}", j);

  // The total number of submitted, approved and rejected applications per research area

  // For each of the past 12 months:
  // - the total submitted, approved, and rejected applications in each month

  // - the sum of funding we approved in each month based on the applications data.

  // The average time in (days) between an application being received (submitted) and an outcome (approved or rejected)

  // A list of application ids which have not been actioned in more than 60 days from their submitted date (i.e. they are still in the submitted state).

  // Return bytes object for HTTP Response

  let output = ResearchArea {
    research_area: "test".to_string(),
    submitted: 1,
    approved: 2,
    rejected: 3,
  };

  let output = serde_json::to_string(&output);
  
  output
}

// DEFINE OUTPUT STRUCT

#[derive(Serialize)]
pub struct ResearchArea {
  research_area: String,
  submitted: u32,
  approved: u32,
  rejected: u32,
}

pub struct Month {
  month: String,
  submitted: u32,
  approved: u32,
  rejected: u32,
  total_funding_approved: u64
}

pub struct Output {
  totals_by_research_area: Vec<ResearchArea>,
  totals_by_month_year_to_date: Vec<Month>,
  mean_response_time_days: u8,
  overdue_unactioned_application_ids: Vec<String>
}