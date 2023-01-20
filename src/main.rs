mod observations;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    observations::get_observations().await?;
    Ok(())
}
