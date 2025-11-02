slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;

    ui.on_prev_page(|| {
        println!("called: prev-page()");
    });

    ui.on_next_page(|| {
        println!("called: next-page()");
    });

    ui.run()?;

    Ok(())
}
