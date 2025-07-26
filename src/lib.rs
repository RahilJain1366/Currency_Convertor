use std::collections::HashMap;

mod utils;

/// Convert currency from one denomination to another using the provided exchange rates.
///
/// # Arguments
///
/// * `base` - The base currency for the exchange rates.
/// * `rates` - A map of currency exchange rates relative to the base currency.
/// * `from` - The currency to convert from.
/// * `to` - The currency to convert to.
/// * `amount` - The amount of money to convert.
///
/// # Returns
///
/// Returns the converted amount of money if successful, or an error message if not.
pub fn convert_currency(
    base: &str,
    rates: &HashMap<String, f64>,
    from: &str,
    to: &str,
    amount: f64,
) -> Result<f64, String> {
    let rate = utils::get_currency_rates(base, &std::sync::Mutex::new(rates.clone()), from, to)?;
    Ok(amount * rate)
}

