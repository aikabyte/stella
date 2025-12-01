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
    #[error("failed to load the file from disk")]
    OpenFailed(#[from] std::io::Error),

    #[error("failed to read the archive file. could be corrupt")]
    ExtractionFailed(#[from] zip::result::ZipError),

    #[error("tokio JoinError (retry the operation")]
    JoinHandleError(#[from] tokio::task::JoinError),
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
        let archive = Arc::new(Mutex::new(
            task::spawn_blocking(|| {
                let file = BufReader::new(File::open(path)?);
                ZipArchive::new(file)
            })
            .await??,
        ));

        let pages = task::spawn_blocking({
            let archive = Arc::clone(&archive);
            async move || {
                let mut archive = archive.lock().await;
                let is_image_file = |name: &str| {
                    let n = name.to_lowercase();
                    n.ends_with("png")
                        || n.ends_with("jpg")
                        || n.ends_with("jpeg")
                        || n.ends_with("webp")
                };

                let mut entries = Vec::new();
                for i in 0..archive.len() {
                    let file = archive.by_index(i);
                    if let Ok(file) = file {
                        let name = file.name().to_string();
                        if is_image_file(&name) {
                            entries.push(name)
                        }
                    }
                }
                // TODO: sort
                entries
            }
        })
        .await?
        .await;

        let page_count = pages.len();

        Ok(Self {
            archive,
            page_count,
            pages,
        })
    }

    async fn load_page(&mut self, logical_index: usize) -> Result<ComicPage, ArchiveError> {
        let buffer = task::spawn_blocking({
            let archive = Arc::clone(&self.archive);
            async move || {
                let mut archive = archive.lock().await;
                let mut entry = archive.by_index(logical_index);
                if let Ok(entry) = archive.by_index(logical_index) {
                    let mut buffer = Vec::new();
                    entry.read_to_end(&mut buffer);
                    Ok(buffer)
                } else {
                    todo!()
                }
            }
        })
        .await?
        .await;

        Ok(ComicPage::new(buffer))
    }

    fn num_pages(&self) -> usize {
        self.page_count
    }
}
