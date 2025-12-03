use crate::{
    app::comic::{ComicPage, ProcessedComicPage},
    core::configuration::RendererSettings,
};
use image::ImageBuffer;
use std::sync::Arc;
use thiserror::Error;
use tokio::{sync::Mutex, task};

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Tokio JoinError. Retry the operation.")]
    JoinHandleError(#[from] tokio::task::JoinError),

    #[error("Unsupported image format.")]
    UnsupportedImageFormat(#[from] image::error::ImageError),
}

type ImgBuf = ImageBuffer<image::Rgba<u8>, Vec<u8>>;

#[allow(unused)]
pub struct Renderer {
    settings: Arc<Mutex<RendererSettings>>,
}

#[allow(unused)]
impl Renderer {
    pub fn new(settings: Option<RendererSettings>) -> Self {
        Self {
            settings: Arc::new(Mutex::new(settings.unwrap_or_default())),
        }
    }

    pub async fn decode(&self, image: ComicPage) -> Result<ProcessedComicPage, RendererError> {
        task::spawn_blocking({
            let settings = Arc::clone(&self.settings);
            move || {
                let mut img = image::load_from_memory(image.bytes())?.to_rgba8();
                Renderer::apply_transforms(&mut img, settings);
                let (width, height) = (img.width(), img.height());

                Ok(ProcessedComicPage::new(img.into_raw(), width, height))
            }
        })
        .await?
    }

    pub fn apply_transforms(img: &mut ImgBuf, settings: Arc<Mutex<RendererSettings>>) {
        let settings = settings.blocking_lock();

        if settings.grayscale() {
            for pixel in img.pixels_mut() {
                let [r, g, b, a] = pixel.0;
                let gray = (r as u16 * 77 + g as u16 * 150 + b as u16 * 29) >> 8;
                pixel.0 = [gray as u8, gray as u8, gray as u8, a];
            }
        }
    }
}
