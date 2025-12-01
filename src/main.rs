#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();

mod app;
mod core;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    ui.run()?;
    Ok(())
}
