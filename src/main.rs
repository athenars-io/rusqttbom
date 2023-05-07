mod forecasts;
mod observations;
mod warning;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    observations::get_observations().await?;
    forecasts::get_forecasts().await?;
    warning::get_warnings().await?;

    Ok(())
}
