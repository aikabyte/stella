// ArchiveHandler opens an archive to extract individual pages as a Vec<u8>.

use async_zip::tokio::read::seek::ZipFileReader;
use std::{error::Error, path::PathBuf};
use tokio::{fs::File, io::BufReader};

pub struct ArchiveHandler {
    archive_path: PathBuf,
    num_pages: usize,
}

impl ArchiveHandler {
    pub async fn new(archive_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = BufReader::new(File::open(&archive_path).await?);
        let zip = ZipFileReader::with_tokio(&mut file).await?;
        let num_pages = zip.file().entries().iter().count();

        Ok(Self {
            archive_path,
            num_pages,
        })
    }

    pub async fn get_page(&self, page_num: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = BufReader::new(File::open(&self.archive_path).await?);
        let mut zip = ZipFileReader::with_tokio(&mut file).await?;

        let mut reader = zip.reader_with_entry(page_num).await?;
        let buf = &mut vec![];
        reader.read_to_end_checked(buf).await?;

        Ok(buf.to_owned())
    }

    pub fn get_num_pages(&self) -> usize {
        self.num_pages
    }
}
