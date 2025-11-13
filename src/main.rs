mod app;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;

    app.run()?;

    Ok(())
}
