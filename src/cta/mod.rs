use gtfs_structures::{Gtfs, GtfsReader};
use std::sync::OnceLock;
pub mod stations;
pub mod traintracker;

static gtfs_url: &str = "https://www.transitchicago.com/downloads/sch_data/google_transit.zip";
static gtfs: OnceLock<Gtfs> = OnceLock::new();

pub async fn load_gtfs() {
  if gtfs.get().is_some() {return;}
  println!("Starting GTFS Data Load.");
  let reader = GtfsReader { read_stop_times: false, read_shapes: false, unkown_enum_as_default: true, trim_fields: true };
  gtfs.set(gtfs_structures::GtfsReader::read_from_url_async(reader, gtfs_url).await.expect("Error loading GTFS data"));
  let example = gtfs.get().unwrap().get_route("1").expect("Invalid Route ID.").long_name.as_ref().expect("No Long Name").to_string();
  println!("GTFS Data Loaded.");
}


pub async fn get_route_name(id: &str) -> Option<String> {
  load_gtfs().await;
  Some(gtfs.get()
    .expect("Error getting GTFS data.")
    .get_route(id)
    .expect("Invalid route ID.")
    .long_name.as_ref()
    .expect("Route has no long name")
    .to_string())
}

pub async fn get_stop_name(id: i32) -> Option<String> {
  load_gtfs().await;
  gtfs.get()?.get_stop(&id.to_string()).ok()?.clone().name
}

