#[allow(unused)]
pub struct ComicPage {
    bytes: Vec<u8>,
}

#[allow(unused)]
impl ComicPage {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
