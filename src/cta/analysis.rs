use super::traintracker;
use super::traintracker::LRouteCode;
use super::traintracker::LRouteName;
use super::traintracker::TTRoute;
pub struct AnomalousTrain {
  anomaly_type: AnomalyType,
  position: traintracker::TTPosition,
}
pub enum AnomalyType {
  RunNumber,
  NextStation,
}
pub async fn check_route(routes: Vec<TTRoute>) -> Vec<AnomalousTrain> {
  // load_stations().await;
  // routes.iter().fold(Vec::new(), |mut acc, rt| {
  //   acc
  // })
  routes.iter().flat_map(check_run_numbers).collect()
}

fn check_run_numbers(route: &TTRoute) -> Vec<AnomalousTrain> {
  route
    .trains
    .iter()
    .filter_map(|train| match route.name {
      LRouteCode::Blue => {
        if !matches!(&train.run_number, 100..=299) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::Brn => {
        if !matches!(&train.run_number, 400..=499) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::G => {
        if !matches!(&train.run_number, 0..=99 | 600..=699) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::Org => {
        if !matches!(&train.run_number, 700..=799) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::P => {
        if !matches!(&train.run_number, 500..=589) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::Pink => {
        if !matches!(&train.run_number, 300..=399) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::Red => {
        if !matches!(&train.run_number, 800..=999) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
      LRouteCode::Y => {
        if !matches!(&train.run_number, 590..=599) {
          Some(AnomalousTrain {
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone(),
          })
        } else {
          None
        }
      }
    })
    .collect()
}

// fn check_next_stations(route: &TTRoute) -> Vec<AnomalousTrain> {
//   route.trains.iter().filter_map(|train| {

//   })
// }
