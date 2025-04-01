use super::gtfs;
use super::stations;
use super::traintracker::LRouteName;
use super::traintracker::TTRoute;
use super::traintracker::LRouteCode;
use super::stations::*;
pub struct AnomalousTrain {
  run_number: String,
  line_name: LRouteName,
  anomaly_type: AnomalyType,
}
pub enum AnomalyType {
  RunNumber,
  NextStation,
}
pub async fn check_route(routes: Vec<TTRoute>) -> Vec<AnomalousTrain> {
  // load_stations().await;
  let anomalous: Vec<AnomalousTrain> = Vec::new();
  routes.into_iter().for_each(|rt| {
    
  });
  anomalous
}

fn check_run_numbers(route: TTRoute) -> Vec<AnomalousTrain> {
  let anomalous: Vec<AnomalousTrain> = Vec::new();
  route.train.into_iter().for_each(|train: super::traintracker::TTPosition| {
    match route.name {
      LRouteCode::Blue => {
        
      },
      LRouteCode::Brn => {

      },
      LRouteCode::G => {

      },
      LRouteCode::Org => {

      },
      LRouteCode::P => {

      },
      LRouteCode::Pink => {

      },
      LRouteCode::Red => {

      },
      LRouteCode::Y => {

      }
    }
  });
  anomalous
}