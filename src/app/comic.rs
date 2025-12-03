#[allow(unused)]
pub struct ComicPage {
    bytes: Vec<u8>,
}

#[allow(unused)]
impl ComicPage {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

#[allow(unused)]
pub struct ProcessedComicPage {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

#[allow(unused)]
impl ProcessedComicPage {
    pub fn new(rgba: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            rgba,
            width,
            height,
        }
    }

    pub fn rgba(&self) -> &Vec<u8> {
        &self.rgba
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
