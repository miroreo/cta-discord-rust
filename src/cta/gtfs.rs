use gtfs_structures::Gtfs;
use std::{env, fs, path::Path};

static GTFS_URL: &str = "https://www.transitchicago.com/downloads/sch_data/google_transit.zip";
static DOCKER_GTFS_PATH: &str = "/data/google_transit.zip";

#[derive(Default)]
pub struct CtaGTFS {
  pub gtfs_data: Gtfs,
  cache_dir: Option<String>,
}

impl CtaGTFS {
  pub async fn new() -> Self {
    let cache_directory = env::var("CACHE_DIRECTORY");
    // check if the cache directory is set and has full read-write permissions
    let has_cache_dir = match cache_directory.clone() {
      Ok(val) => {
        let path = Path::new(val.as_str());
        println!("Found CACHE_DIRECTORY of {val}. Checking permissions...");
        let has_read_write = match fs::metadata(path) {
          Ok(data) => {
            let permissions = data.permissions();
            if permissions.readonly() {
              println!("Cache Directory '{val}' does not have write permission.");
              false
            } else {
              println!("Cache Directory '{val}' has write permission.");
              true
            }
          }
          Err(e) => {
            println!("Error getting metadata for Cache Directory: {e}");
            false
          }
        };
        has_read_write
      }
      Err(e) => {
        println!("Cache directory not set. Not caching GTFS data.");
        false
      }
    };
    let has_cached_data = if has_cache_dir {
      let dir = cache_directory.clone().unwrap();
      let data_loc = format!("{dir}/google_transit.zip");
      let data_path = Path::new(&data_loc);
      match fs::exists(data_path) {
        Ok(exists_val) => {
          if exists_val {
            println!("Found cached data.");
          }
          exists_val
        }
        Err(e) => {
          println!("Could not find cached data.");
          false
        }
      }
    } else {
      false
    };

    if has_cached_data {
      return Self {
        gtfs_data: Self::load_from_cache(&cache_directory.clone().unwrap()).await,
        cache_dir: Some(cache_directory.unwrap().clone()),
      };
    } else if has_cache_dir {
      return Self {
        gtfs_data: Self::cache_from_web(&cache_directory.clone().unwrap()).await,
        cache_dir: Some(cache_directory.unwrap().clone()),
      };
    }
    Self {
      gtfs_data: Self::load_from_web().await,
      cache_dir: None,
    }
  }

  async fn load_from_web() -> Gtfs {
    let reader = gtfs_structures::GtfsReader::default()
      .read_stop_times(false)
      .read_shapes(false)
      .unkown_enum_as_default(true)
      .trim_fields(true);
    reader
      .read_from_url_async(GTFS_URL)
      .await
      .expect("Error downloading GTFS data. ")
  }
  async fn cache_from_web(cache_directory: &str) -> Gtfs {
    let reader = gtfs_structures::GtfsReader::default()
      .read_stop_times(false)
      .read_shapes(false)
      .unkown_enum_as_default(true)
      .trim_fields(true);
    let path = format!("{cache_directory}/google_transit.zip");
    let resp = reqwest::get(GTFS_URL)
      .await
      .expect("GTFS Caching request failed");
    let body = resp.bytes().await.expect("GTFS response body invalid");
    // let mut out = std::fs::OpenOptions::new().write(true).truncate(true).open(path.clone())
    // std::io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy GTFS content to cache");
    fs::write(path.clone(), body).expect("Error overwriting GTFS data.");
    reader
      .read_from_path(path)
      .expect("Could not load GTFS data.")
  }
  async fn load_from_cache(cache_directory: &str) -> Gtfs {
    let reader = gtfs_structures::GtfsReader::default()
      .read_stop_times(false)
      .read_shapes(false)
      .unkown_enum_as_default(true)
      .trim_fields(true);
    let cached_data_loc = format!("{}/google_transit.zip", cache_directory);
    match reader.read_from_path(cached_data_loc) {
      Ok(data) => data,
      Err(e) => {
        println!("Invalid GTFS data encountered. Refreshing the cache.");
        Self::cache_from_web(cache_directory).await
      }
    }
  }

  pub async fn reload_gtfs(&self) {
    match self.cache_dir.clone() {
      Some(dir) => {
        Self::cache_from_web(&dir).await;
      }
      None => {
        println!("Not attempting to reload cache: Cache Directory does not exist.");
      }
    }
  }

  pub fn route_ids(&self) -> Vec<String> {
    self
      .gtfs_data
      .routes
      .keys()
      .map(|r| r.to_string())
      .collect()
  }

  pub fn get_route_name(&self, id: &str) -> std::string::String {
    // load_gtfs().await;
    self
      .gtfs_data
      .get_route(id)
      .expect("Invalid route ID.")
      .long_name
      .as_ref()
      .expect("Route has no long name")
      .to_string()
  }

  pub fn search_stops(&self, search: &str) -> Option<Vec<String>> {
    let mut found_stops: Vec<String> = Vec::new();
    self.gtfs_data.stops.clone().into_values().for_each(|stop| {
      if stop.name.clone().unwrap_or(String::new()).contains(search) {
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
