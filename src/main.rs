use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::time::{sleep, Duration};
use warp::Filter;
use chrono;

#[derive(Deserialize)]
struct ApiResponse {
    data: CurrencyData,
}

#[derive(Deserialize)]
struct CurrencyData {
    #[serde(rename = "EUR")]
    eur: Currency,
}

#[derive(Deserialize)]
struct Currency {
    value: f64,
}

struct State {
    euro_rate: Mutex<f64>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY").expect("Environment variable API_KEY is not set");

    let refresh_interval_secs = env::var("REFRESH_INTERVAL_SECS")
        .unwrap_or_else(|_| "21600".to_string())
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid SLEEP_DURATION value; using default of 21600 seconds.");
            21600
        });

    let state = Arc::new(State {
        euro_rate: Mutex::new(0.0),
    });

    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        fetch_exchange_rate(state_clone, api_key, refresh_interval_secs).await;
    });

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C signal");
        println!("Received termination signal, shutting down gracefully. Bye!");
    };

    let metrics_route = warp::path("metrics").map(move || {
        let euro_rate = *state.euro_rate.lock().unwrap();
        format!("euro_usd_rate {}", euro_rate)
    });

    println!("Starting server on port 8080...");
    let (_, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), shutdown_signal);

    server.await;
}

async fn fetch_exchange_rate(state: Arc<State>, api_key: String, refresh_interval_secs: u64) {
    let client = Client::new();
    let url = "https://api.currencyapi.com/v3/latest?currencies[]=EUR";

    loop {
        if let Ok(response) = client.get(url).header("apikey", &api_key).send().await {
            if let Ok(api_response) = response.json::<ApiResponse>().await {
                let rate = api_response.data.eur.value;
                *state.euro_rate.lock().unwrap() = rate;
                println!(
                    "[{}] Refreshed exchange rate: euro_usd_rate = {}",
                    chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    rate
                );
            } else {
                eprintln!("Failed to parse API response JSON");
            }
        } else {
            eprintln!("Failed to fetch exchange rate from API");
        }
        sleep(Duration::from_secs(refresh_interval_secs)).await;
    }
}
