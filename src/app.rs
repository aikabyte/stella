slint::include_modules!();

use std::error::Error;

pub struct App {
    ui_handle: MainWindow,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let ui_handle = MainWindow::new()?;

        Ok(Self { ui_handle })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        self.ui_handle.run()?;

        Ok(())
    }
}
