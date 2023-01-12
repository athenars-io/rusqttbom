mod observations;

#[tokio::main]
async fn main() {
    observations::get_weather()
        .await
        .expect("The get weather function didn't seem to work");
}