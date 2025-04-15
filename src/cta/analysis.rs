use super::traintracker;
use super::traintracker::LRouteName;
use super::traintracker::TTRoute;
use super::traintracker::LRouteCode;
pub struct AnomalousTrain {
  anomaly_type: AnomalyType,
  position: traintracker::TTPosition
}
pub enum AnomalyType {
  RunNumber,
  NextStation,
}
pub async fn check_route(routes: Vec<TTRoute>) -> Vec<AnomalousTrain> {
  // load_stations().await;
  routes.iter().fold(Vec::new(), |mut acc, rt| {
    acc
  })
}

fn check_run_numbers(route: TTRoute) -> Vec<AnomalousTrain> {
  route.train.iter().fold(Vec::new(), |mut acc, train| {
    match route.name {
      LRouteCode::Blue => {
        if !matches!(&train.run_number, 100..=299) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::Brn => {
        if !matches!(&train.run_number, 400..=499) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::G => {
        if !matches!(&train.run_number, 0..=99 | 600..=699) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::Org => {
        if !matches!(&train.run_number, 700..=799) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::P => {
        if !matches!(&train.run_number, 500..=589) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::Pink => {
        if !matches!(&train.run_number, 300..=399) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::Red => {
        if !matches!(&train.run_number, 800..=999) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      },
      LRouteCode::Y => {
        if !matches!(&train.run_number, 590..=599) {
          acc.push(AnomalousTrain{
            anomaly_type: AnomalyType::RunNumber,
            position: train.clone()
          });
        }
      }
    }
    acc
  })
}