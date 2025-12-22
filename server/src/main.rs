mod app;
mod cache;
mod config;
mod errors;
mod github;
mod health_check;
mod programing_languages;
mod state;
mod telemetry;

use anyhow::Error;
use app::App;

use config::get_app_settings;

use telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber(
        String::from("rust-good-first-issue-api"),
        String::from("info"),
    );

    // Initialize tracing subscriber
    init_subscriber(subscriber);

    let settings = get_app_settings().expect("Unable to get server settings");
    let app = App::new(settings.clone()).await?;

    let addr = settings.application.get_addr()?;
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server running on {}", addr);

    axum::serve(listener, app.router).await?;

    Ok(())
}
