use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Options, Transform, Tree};
use svg::Document;
use svg::node::element::{Path, Rectangle, Symbol, Text, Use};
use serde_variant::to_variant_name;
use thiserror::Error;


use resvg::render;

use crate::cta::traintracker::LRouteName;

const OFFSET: f32 = 17.101_563;

#[derive(Clone, Debug)]
pub struct Arrival {
  pub destination_name: String,
  pub route: LRouteName,
  pub train_number: i32,
  pub countdown: String,
  pub is_scheduled: bool
}

#[allow(clippy::cast_precision_loss)]
pub fn train(stop_description: String, arrivals: &[Arrival]) -> Document {
  let canvas_height = 65 + arrivals.len() * 105;

  let mut document = Document::new()
    .set("viewBox", (0,0,1200,canvas_height))
    .set("vertical-align", "top");

  document = document.add(Symbol::new()
    .set("viewBox", (0, 0, 15, 15))
    .set("id", "clockSymbol")
    .add(Path::new()
      .set("d", "m7.50078,15.00156c-4.2,0 -7.5,-3.4 -7.5,-7.5s3.3,-7.5 7.5,-7.5c4.1,0 7.5,3.3 7.5,7.5s-3.4,7.5 -7.5,7.5zm0,-14c-3.6,0 -6.5,2.9 -6.5,6.5s2.9,6.5 6.5,6.5c3.6,0 6.5,-2.9 6.5,-6.5s-2.9,-6.5 -6.5,-6.5z"))
    .add(Path::new()
      .set("d", "m7.00078,8.00156c-0.1,0 -0.1,0 -0.2,-0.1c-0.2,-0.1 -0.3,-0.4 -0.2,-0.6l2.5,-4.7c0.1,-0.2 0.4,-0.3 0.6,-0.2c0.2,0.1 0.3,0.4 0.2,0.6l-2.5,4.7c-0.1,0.2 -0.2,0.3 -0.4,0.3z")
      .set("opacity", 0.6))
    .add(Path::new()
      .set("d", "m7.10078,8.00156c-0.1,0 -0.1,0 0,0c-0.3,0 -0.5,-0.2 -0.5,-0.5l0.4,-5.3c0,-0.3 0.2,-0.5 0.5,-0.4c0.3,0 0.5,0.2 0.4,0.5l-0.4,5.2c0,0.3 -0.2,0.5 -0.4,0.5z")
      .set("opacity", 0.3))
    .add(Path::new()
      .set("d", "m7.10078,8.00156c-0.1,0 -0.3,-0.1 -0.4,-0.2c-0.2,-0.2 -0.1,-0.5 0.1,-0.7l4.3,-3.2c0.2,-0.2 0.5,-0.1 0.7,0.1c0.2,0.2 0.1,0.5 -0.1,0.7l-4.4,3.2c0,0 -0.1,0.1 -0.2,0.1z")));
  document = document.add(Symbol::new()
    .set("viewBox", (0, 0, 20, 20))
    .set("id", "trackingSymbol")
    .add(Path::new()
      .set("d", "M7.7,18c0-3.1-2.5-5.6-5.6-5.7l0,0c-0.2,0-0.4,0-0.6-0.2l0,0v0c-0.3-0.3-0.3-0.7,0-1l0,0C1.6,11,1.7,11,1.9,10.9l0,0c0,0,0,0,0,0h0c0,0,0,0,0,0l0,0c0,0,0.1,0,0.1,0l0,0c3.9,0,7.1,3.2,7.1,7.1l0,0c0,0.4-0.3,0.7-0.7,0.7l0,0C8,18.7,7.7,18.4,7.7,18L7.7,18z M17.9,18C17.9,9.2,10.8,2.1,2,2.1l0,0v0c-0.4,0-0.7-0.3-0.7-0.7l0,0C1.3,1,1.6,0.7,2,0.7l0,0c9.5,0,17.3,7.7,17.3,17.3l0,0c0,0.4-0.3,0.7-0.7,0.7l0,0C18.2,18.7,17.9,18.4,17.9,18L17.9,18z"))
    .add(Path::new()
      .set("d", "M12.8,18C12.8,12.1,8,7.3,2.1,7.2l0,0c-0.3,0-0.6-0.1-0.7-0.4l0,0c-0.2-0.4,0-0.8,0.3-1l0,0c0,0-0.1,0,0.3-0.1l0,0c6.7,0,12.2,5.4,12.2,12.2l0,0c0,0.4-0.3,0.7-0.7,0.7l0,0C13.1,18.7,12.8,18.4,12.8,18L12.8,18z")));
  
  document = document.add(Rectangle::new()
  .set("width", 1200)
  .set("height", canvas_height)
  .set("fill", "#1e1e1e")
  .set("x", 0)
  .set("y", 0));
  document = document.add(Text::new(stop_description)
    .set("x", 25)
    .set("y", 30.0 + OFFSET)
    .set("fill", "#ffffff")
    .set("font-size", 25)
    .set("font-weight", "bold")
    .set("font-family", "Helvetica")
    .set("vertical-align", "top"));
  for (i, arr) in arrivals.iter().enumerate() {
    let line_color = match arr.route {
        LRouteName::Red => "#c60c30",
        LRouteName::P => "#522398",
        LRouteName::Y => "#f9e300",
        LRouteName::Blue => "#00a1de",
        LRouteName::Pink => "#e27ea6",
        LRouteName::G => "#009b3a",
        LRouteName::Org => "#f9461c",
        LRouteName::Brn => "#62361b",
    };
    let inverse = ["Cottage Grove", "UIC-Halsted"].contains(&arr.destination_name.clone().as_str());
    let black_text = arr.route.eq(&LRouteName::Y);
    let bg = if inverse {"#ffffff"} else {line_color};
    let fg = if inverse {line_color} else if black_text {"#000000"} else {"#ffffff"};

    document = document.add(Rectangle::new()
      .set("width", 1200)
      .set("height", 100)
      .set("fill", bg)
      .set("x", 0)
      .set("y", 65 + (i*105)));

    document = document.add(Text::new(arr.destination_name.clone())
      .set("x", 25)
      .set("y", 115.0 + ((i as f32)*105.0) + OFFSET)
      .set("font-size", 48)
      .set("font-weight", "bold")
      .set("font-family", "Helvetica")
      .set("vertical-align", "top")
      .set("border", "1px")
      .set("fill", fg));

    document = document.add(Text::new(format!("{} #{:0>3} to", to_variant_name(&arr.route).unwrap(), arr.train_number.to_string()))
      .set("x", 25)
      .set("y", 75.0 + ((i as f32)*105.0) + OFFSET)
      .set("font-size", 20)
      .set("font-family", "Helvetica")
      .set("vertical-align", "top")
      .set("fill", fg));

    document = document.add(Text::new(&arr.countdown)
      .set("x", 1130)
      .set("y", 115.0 + ((i as f32)*105.0) + OFFSET)
      .set("font-size", 48)
      .set("font-weight", "bold")
      .set("text-anchor", "end")
      .set("font-family", "Helvetica")
      .set("vertical-align", "top")
      .set("fill", fg));

    document = document.add(Use::new()
      .set("href", if arr.is_scheduled {"#clockSymbol"} else { "#trackingSymbol" })
      .set("width", 25)
      .set("height", 25)

      .set("fill", fg)
      .set("x", 1150)
      .set("y", 100 + (i * 105)));
  }
  document
}

// pub fn m() {
//   let doc = &train("Upcomining Arrivals at Halsted (Orange Line)".to_string(), 
//     Vec::<Arrival>::from([Arrival{
//     countdown: "Due".to_string(),
//     destination_name: "Loop".to_string(),
//     route: LRouteName::Org,
//     train_number: 715,
//     is_scheduled: false
//   },
//   Arrival{
//     countdown: "5 min".to_string(),
//     destination_name: "Midway".to_string(),
//     route: LRouteName::Org,
//     train_number: 708,
//     is_scheduled: false
//   },
//   Arrival{
//     countdown: "12 min".to_string(),
//     destination_name: "Loop".to_string(),
//     route: LRouteName::Org,
//     train_number: 717,
//     is_scheduled: false
//   },
//   Arrival{
//     countdown: "13 min".to_string(),
//     destination_name: "Midway".to_string(),
//     route: LRouteName::Org,
//     train_number: 709,
//     is_scheduled: false
//   },
//   Arrival{
//     countdown: "20 min".to_string(),
//     destination_name: "Midway".to_string(),
//     route: LRouteName::Org,
//     train_number: 716,
//     is_scheduled: false
//   },
//   Arrival{
//     countdown: "28 min".to_string(),
//     destination_name: "Midway".to_string(),
//     route: LRouteName::Org,
//     train_number: 715,
//     is_scheduled: true
//   },
//   Arrival{
//     countdown: "40 min".to_string(),
//     destination_name: "Midway".to_string(),
//     route: LRouteName::Org,
//     train_number: 717,
//     is_scheduled: false
//   }]));
//   svg::save("./arr.svg", doc);
//   render_doc(doc.clone());
// }

#[derive(Error, Debug)]
pub enum ArrivalDisplayError {
  #[error("Error encoding SVG into PNG image.")]
  EncodingError(#[from] png::EncodingError),
  #[error("Error loading files")]
  FileError(#[from] std::io::Error)
}
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn render_doc(doc: &svg::Document) -> Result<Vec<u8>, ArrivalDisplayError> {
  let mut font_database = fontdb::Database::new();
  match font_database.load_font_file("./src/arrivaldisplay/ttf/Helvetica.ttf") {
    Ok(()) => {},
    Err(e) => {
      return Err(ArrivalDisplayError::FileError(e));
    }
  }
  match font_database.load_font_file("./src/arrivaldisplay/ttf/Helvetica-Bold.ttf") {
    Ok(()) => {},
    Err(e) => {
      return Err(ArrivalDisplayError::FileError(e));
    }
  }
  match font_database.load_font_file("./src/arrivaldisplay/ttf/Helvetica-Light.ttf") {
    Ok(()) => {},
    Err(e) => {
      return Err(ArrivalDisplayError::FileError(e));
    }
  }
  let options = Options{fontdb: std::sync::Arc::from(font_database), ..Default::default()};

  let tree = Tree::from_str(doc.to_string().as_str(), &options).expect("Could not fetch Tree from SVG Document");
  // tree.postprocess(PostProcessingSteps::default(), &fontdb);
  let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).expect("Couldn't create a new pixmap");
  render(&tree, Transform::default(), &mut pixmap.as_mut());
  // pixmap.save_png("./arr2.png");
  match pixmap.encode_png() {
    Ok(data) => {
      Ok(data)
    },
    Err(err) => {
      Err(ArrivalDisplayError::EncodingError(err))
    }
  }
}