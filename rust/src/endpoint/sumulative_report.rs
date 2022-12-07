use std::{collections::HashMap};
use serde_json::{Value, Result};
use serde::Serialize;

// CREATE SUMULATIVE REPORT
pub fn create_sumulative_report(data: &actix_web::web::Bytes) -> Result<String> {
  // log::info!("{}", str::from_utf8(data).unwrap());

  // get json from data
  let json: Value = serde_json::from_slice(data).unwrap();

  // setup
  let mut known_research_areas = vec![];

  let mut output = Output {
    totals_by_research_area: HashMap::new(),
    totals_by_month_ytd: HashMap::new(),
    mean_response_time_days: 0,
    overdue_unactioned_application_ids: vec![]
  };


  for i in 0..json["items"].as_array().expect("Array Expected").len() {

      // The total number of submitted, approved and rejected applications per research area
      let research_area = serde_json::to_string(&json["items"][i]["research_area"]).unwrap();
      

      let tmp = research_area.clone();
      let tmp2 = research_area.clone();

      if !known_research_areas.contains(&research_area) {
        // add to known research areas
        known_research_areas.push(tmp);

        output.totals_by_research_area.insert(tmp2, ResearchArea{
          submitted: 0,
          approved: 0,
          rejected:0
        });

      };

      // output.totals_by_research_area[&research_area].submitted += 1;

      // log::info!("{}", research_area);
  }

  // The total number of submitted, approved and rejected applications per research area

  // For each of the past 12 months:
  // - the total submitted, approved, and rejected applications in each month

  // - the sum of funding we approved in each month based on the applications data.

  // The average time in (days) between an application being received (submitted) and an outcome (approved or rejected)

  // A list of application ids which have not been actioned in more than 60 days from their submitted date (i.e. they are still in the submitted state).

  // Return bytes object for HTTP Response

  // let research_area = ResearchArea {
  //   research_area: "test".to_string(),
  //   submitted: 1,
  //   approved: 2,
  //   rejected: 3,
  // };

  // let month = Month {
  //   month: "01".to_string(),
  //   submitted: 1,
  //   approved: 2,
  //   rejected: 3,
  //   total_funding_approved: 2000
  // };

  // let research_areas = vec![research_area];
  // let months = vec![month];
  // let overdue_unactioned_application_ids = vec!["1".to_string(),"2".to_string(),"3".to_string()];

  // let output = Output {
  //   totals_by_research_area: research_areas,
  //   totals_by_month_ytd: months,
  //   mean_response_time_days: 4,
  //   overdue_unactioned_application_ids: overdue_unactioned_application_ids
  // };

  let output = serde_json::to_string(&output);
  
  output
}

// DEFINE OUTPUT STRUCT

// struct WellcomeFundingApplicationItem {
//   application_id: String, // e.g. "7d292f50-675d-4380-a094-230c426b8eb0",
//   lead_applicant_name: String, // e.g. "Robert Roberts",
//   lead_applicant_email: String, // e.g. "frogers@example.org",
//   lead_applicant_address: String, // e.g. "654 Martin Pike\nVictoriatown, NE 01985",
//   organisation_name: String, // e.g. "Dorsey, Bennett and Torres",
//   summary: String, // e.g. "Process though wind operation",
//   amount_awarded: u32, // e.g. 6916376,
//   research_area: String, // e.g. "climate",
//   status: String, // e.g. "rejected",
//   submitted_date: String, // e.g. "2021-12-11",
//   actioned_date: String, // e.g. "2021-12-11"
// }

#[derive(Serialize)]
pub struct ResearchArea {
  submitted: u32,
  approved: u32,
  rejected: u32
}

#[derive(Serialize)]
pub struct Month {
  submitted: u32,
  approved: u32,
  rejected: u32,
  total_funding_approved: u64
}

#[derive(Serialize)]
pub struct Output {
  totals_by_research_area: HashMap<String, ResearchArea>,
  totals_by_month_ytd: HashMap<String, Month>,
  mean_response_time_days: u8,
  overdue_unactioned_application_ids: Vec<String>
}