mod forecasts;
mod observations;
mod warning;

#[tokio::main]
async fn main() {
    match observations::get_observations().await {
        Ok(_) => println!("observations ok"),
        Err(_) => println!("observations failed"),
    }
    match forecasts::get_forecasts().await {
        Ok(_) => println!("forecasts ok"),
        Err(_) => println!("forecasts failed"),
    }
    match warning::get_warnings().await {
        Ok(_) => println!("warning ok"),
        Err(_) => println!("warning failed"),
    }
}
