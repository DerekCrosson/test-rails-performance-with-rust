#[macro_use]
extern crate helix;

extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::path::Path;
use imageproc::drawing::{
  draw_text_mut,
  draw_hollow_circle_mut
};
use image::{Rgb, RgbImage};
use rusttype::{FontCollection, Scale};

ruby! {
    class ImageStamper {
        def stamp(message: String, outfile: String) {
          let public_path = &[String::from("public"), outfile].join("/");
          let out_path = Path::new(&public_path);

          let mut image = RgbImage::new(200, 200);
          let grey = Rgb([220u8, 220u8, 220u8]);

          draw_hollow_circle_mut(&mut image, (85, 100), 75, grey);

          let font = Vec::from(include_bytes!("Raleway-Black.ttf") as &[u8]);
          let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

          let height = 48.0;
          let scale = Scale { x: height * 1.2, y: height };

          draw_text_mut(&mut image, grey, 50, 75, scale, &font, &message);
          let _result = image.save(out_path).unwrap();
        }
    }
}
