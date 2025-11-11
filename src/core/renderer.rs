// This pipeline takes raw image data, applies the requested image transforms, then returns a Slint Image.

use slint::{Image, Rgba8Pixel as Pixel, SharedPixelBuffer as PixelBuf};
use std::error::Error;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn decode(&self, raw_image: Vec<u8>) -> Result<Image, Box<dyn Error>> {
        let img = image::load_from_memory(&raw_image)?.into_rgba8();

        let buf = PixelBuf::<Pixel>::clone_from_slice(img.as_raw(), img.width(), img.height());

        Ok(Image::from_rgba8(buf))
    }
}
