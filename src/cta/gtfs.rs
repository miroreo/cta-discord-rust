use gtfs_structures::{Gtfs, GtfsReader};
use std::{env, fs, sync::OnceLock};

static gtfs_url: &str = "https://www.transitchicago.com/downloads/sch_data/google_transit.zip";
static DOCKER_GTFS_PATH: &str = "/data/google_transit.zip";

#[derive(Default)]
pub struct CtaGTFS {
  pub gtfs_data: Gtfs,
}

impl CtaGTFS {
  pub async fn new() -> Self {
    let new_self: Self;

    let reader = gtfs_structures::GtfsReader::default()
      .read_stop_times(false)
      .read_shapes(false)
      .unkown_enum_as_default(true)
      .trim_fields(true);
  
    match env::var("DEVELOPMENT") {
      Ok(val) => {
        if val.eq("1") {
          println!("Detected Development Environment. Using local GTFS data.");
          // new_self.gtfs_data = 
          println!("GTFS data loaded.");
          return Self {
            gtfs_data: reader.read_from_path("./google_transit.zip").expect("Could not find valid GTFS data at ./google_transit.zip")
          };
        }
      }
      Err(e) => {
        println!("Error in GTFS development detection. {}", e);
      }
    }
    match fs::exists(DOCKER_GTFS_PATH) {
      Ok(true) => {
        println!("GTFS data found in docker image. Loading data now.");
        return Self {
          gtfs_data: reader.read_from_path(DOCKER_GTFS_PATH).expect("Could not find valid GTFS data in the Docker image.")
        };
      }
      Ok(false) => {
  
      }
      Err(err) => {
        println!("Error detecting docker GTFS")
      }
    }
    println!("Starting web GTFS Data Load.");
    // new_self.gtfs_data =
    // println!("GTFS Data Loaded.");
    // new_self
    Self {
      gtfs_data: gtfs_structures::GtfsReader::read_from_url_async(reader, gtfs_url).await.expect("Error downloading GTFS data. ")
    }
  }
  
  pub async fn reload_gtfs() {
    let path = if env::var("DEVELOPMENT").ok().eq(&Some("1".to_string())) { "./google_transit.zip" } else {"/data/google_transit.zip"};
    println!("GTFS data reload has been requested. App will restart.");
    let resp = reqwest::get(gtfs_url).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut out = std::fs::OpenOptions::new().write(true).truncate(true).open(path).expect("Could not get gtfs file to overwrite.");
    std::io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    panic!("exiting to refresh GTFS data loaded.");
  }

  pub fn get_route_name(&self, id: &str) -> Option<String> {
    // load_gtfs().await;
    Some(self.gtfs_data
      .get_route(id)
      .expect("Invalid route ID.")
      .long_name.as_ref()
      .expect("Route has no long name")
      .to_string())
  }

  pub fn search_stops(&self, search: &str) -> Option<Vec<String>> {
    let mut found_stops: Vec<String> = Vec::new();
    self.gtfs_data.stops.clone().into_values().into_iter().for_each(|stop| {
      if stop.name.clone().unwrap_or("".to_string()).contains(search) {
        found_stops.push(stop.id.clone());
      }
    });
    Some(found_stops).or(None)
  }
  
  pub fn get_stop_name(&self, id: i32) -> Option<String> {
    // load_gtfs().await;
    self.gtfs_data.get_stop(&id.to_string()).ok()?.clone().name
  }
}




