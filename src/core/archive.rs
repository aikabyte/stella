use crate::app::comic::ComicPage;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::Arc,
};
use thiserror::Error;
use tokio::{sync::Mutex, task};
use zip::ZipArchive;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("Failed to parse the archive. Error: {0}")]
    ExtractionFailed(#[from] zip::result::ZipError),

    #[error("Tokio JoinError. Retry the operation.")]
    JoinHandleError(#[from] tokio::task::JoinError),

    #[error("Failed to read the file from disk. Error: {0}")]
    OpenFailed(#[from] std::io::Error),
}

#[allow(unused)]
pub trait ArchiveHandler {
    /// Open and index the archive.
    async fn open(path: PathBuf) -> Result<Self, ArchiveError>
    where
        Self: Sized;

    /// Load a single page into memory.
    async fn load_page(&mut self, logical_index: usize) -> Result<ComicPage, ArchiveError>;

    /// Get the total number of *image* pages.
    fn num_pages(&self) -> usize;
}

type CbzArchive = ZipArchive<BufReader<File>>;

#[allow(unused)]
pub struct Cbz {
    archive: Arc<Mutex<CbzArchive>>,
    page_count: usize,
    pages: Vec<String>,
}

impl ArchiveHandler for Cbz {
    async fn open(path: PathBuf) -> Result<Self, ArchiveError> {
        task::spawn_blocking(|| {
            let reader = BufReader::new(File::open(path)?);
            let archive = Arc::new(Mutex::new(ZipArchive::new(reader)?));

            let pages = {
                let is_image_file = |name: &str| {
                    let n = name.to_lowercase();
                    n.ends_with("png") || n.ends_with("jpg") || n.ends_with("jpeg")
                };

                let mut images: Vec<String> = archive
                    .blocking_lock()
                    .file_names()
                    .filter(|name| is_image_file(name))
                    .map(|name| name.to_string())
                    .collect();

                images.sort();
                images
            };

            let page_count = pages.len();

            Ok(Self {
                archive,
                page_count,
                pages,
            })
        })
        .await?
    }

    async fn load_page(&mut self, logical_index: usize) -> Result<ComicPage, ArchiveError> {
        task::spawn_blocking({
            let archive = Arc::clone(&self.archive);
            let page_name = self.pages[logical_index - 1].clone();

            move || {
                let mut archive = archive.blocking_lock();
                let mut buffer = Vec::new();

                if let Ok(mut entry) = archive.by_name(&page_name) {
                    let _ = entry.read_to_end(&mut buffer);
                }

                Ok(ComicPage::new(buffer))
            }
        })
        .await?
    }

    fn num_pages(&self) -> usize {
        self.page_count
    }
}
