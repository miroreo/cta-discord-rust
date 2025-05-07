use bustracker::BusTracker;
use gtfs::CtaGTFS;
use traintracker::TrainTracker;


pub mod stations;
pub mod traintracker;
pub mod gtfs;
pub mod bustracker;
pub mod analysis;

struct CtaOptions<'a> {
  traintracker_token: &'a str,
  bustracker_token: &'a str,
}
struct CTA {
  traintracker: TrainTracker,
  bustracker: BusTracker,
  gtfs: CtaGTFS
}
impl CTA {
  pub async fn new(options: CtaOptions<'_>) -> Self {
    let tt = TrainTracker::new(options.traintracker_token);
    let bt = BusTracker::new(options.bustracker_token);
    let cta_gtfs = CtaGTFS::new().await;

    Self {
      traintracker: tt,
      bustracker: bt,
      gtfs: cta_gtfs,
    }
  }
}