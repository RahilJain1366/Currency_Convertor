use std::collections::HashMap;
use std::sync::Mutex;
use std::io::{self, Write};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use currency_convertor::convert_currency;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    base: Option<String>,
    rates: HashMap<String, f64>,
    date: String,
    success: bool,
}

#[derive(Deserialize, Debug)]
struct ApiError{
    success: bool,
    error: ErrorDetails,
}

#[derive(Deserialize, Debug)]
struct ErrorDetails {
    code: u32,
    info: String,
}

async fn fetch_currency_rates(_base: &str, api_key: &str) -> Result<(String, Mutex<HashMap<String, f64>>), Box<dyn std::error::Error>> {
    // Note: Free plans often only support EUR as base currency
    let url = format!(
        "http://api.exchangeratesapi.io/v1/latest?access_key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .send()
        .await?;

    // Print the raw response body for debugging
    let body = res.text().await?;
    println!("Raw response body: {}", body);

    // Try to parse as error first
    if let Ok(error) = serde_json::from_str::<ApiError>(&body) {
        return Err(format!("API Error {}: {}", error.error.code, error.error.info).into());
    }
    
    // Check if it's a simple error message
    if body.contains("\"error\"") && !body.contains("\"rates\"") {
        return Err(format!("API Error: {}", body).into());
    }
    
    // Deserialize the response body into ApiResponse
    let api_response: ApiResponse = serde_json::from_str(&body)?;

    if !api_response.success {
        return Err("Failed to fetch currency rates".into());
    }
    let api_base = api_response.base.ok_or("Missing 'base' field in API response")?;

    Ok((api_base, Mutex::new(api_response.rates)))
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer.trim().to_uppercase()
}

fn read_amount(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input");
        
        match buffer.trim().parse::<f64>() {
            Ok(amount) if amount > 0.0 => return amount,
            Ok(_) => eprintln!("Please enter a positive number."),
            Err(_) => eprintln!("Please enter a valid number."),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set in .env");

    if api_key.is_empty() {
        eprintln!("API_KEY is empty. Please set it in the .env file.");
        return;
    }

    let base = read_input("Enter base currency (e.g., EUR): ");
    let from = read_input("Enter 'from' currency (e.g., EUR): ");
    let to = read_input("Enter 'to' currency (e.g., USD): ");
    let amount = read_amount("Enter amount to convert: ");

    match fetch_currency_rates(&base, &api_key).await {
        Ok((api_base, rates)) => {
            let rates_guard = rates.lock().unwrap();
            match convert_currency(&api_base, &*rates_guard, &from, &to, amount) {
                Ok(converted_amount) => {
                    println!("{:.2} {} = {:.2} {}", amount, from, converted_amount, to);
                    
                    // Calculate and display the exchange rate
                    let rate = converted_amount / amount;
                    println!("Exchange rate: 1 {} = {:.4} {}", from, rate, to);
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to fetch currency rates: {}", err),
    }
}
