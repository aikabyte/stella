#[allow(unused)]
pub enum RendererSetting {
    Grayscale(bool),
}

#[allow(unused)]
#[derive(Default)]
pub struct RendererSettings {
    grayscale: bool,
} // Instantiate once, share everywhere.

#[allow(unused)]
impl RendererSettings {
    pub fn load() { /* Deserialize from disk. */
    }

    pub fn reset(mut self) {
        self = Self::default();
        self.save();
    }

    fn save(&self) { /* Serialize to disk. */
    }

    pub fn toggle(&mut self, setting: RendererSetting) {
        match setting {
            RendererSetting::Grayscale(bool) => {
                self.grayscale = bool;
            }
        }
        self.save();
    }
}

impl RendererSettings {
    pub fn grayscale(&self) -> bool {
        self.grayscale
    }
}
