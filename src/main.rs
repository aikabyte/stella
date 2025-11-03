slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;

    ui.run()?;

    Ok(())
}
