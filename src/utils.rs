use std::sync::Mutex;
use std::collections::HashMap;

pub fn get_currency_rates(
    base: &str,
    rates: &Mutex<HashMap<String, f64>>,
    from: &str,
    to: &str,
) -> Result<f64, String> {
    let rates = rates.lock().map_err(|_| "Failed to acquire lock")?;

    if from == base {
        rates.get(to).copied().ok_or_else(|| format!("Rate for '{}' not found", to))
    } else if to == base {
        rates
            .get(from)
            .map(|rate| 1.0 / rate)
            .ok_or_else(|| format!("Rate for '{}' not found", from))
    } else {
        match (rates.get(from), rates.get(to)) {
            (Some(from_rate), Some(to_rate)) => Ok(to_rate / from_rate),
            (None, _) => Err(format!("Rate for '{}' not found", from)),
            (_, None) => Err(format!("Rate for '{}' not found", to)),
        }
    }
}
