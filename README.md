# 💱 Currency Converter

A simple command-line currency converter built using **Rust** that fetches real-time exchange rates and allows users to convert between different global currencies with ease.

## Features

- Convert between major world currencies
- Live exchange rates powered by API
- Lightweight and fast Rust implementation
- Input validation and error handling
- Clean and interactive terminal UI

## Built With

- [Rust](https://www.rust-lang.org/)
- [reqwest](https://crates.io/crates/reqwest) — for HTTP requests
- [serde](https://crates.io/crates/serde) — for JSON parsing
- [dotenv](https://crates.io/crates/dotenv) — to manage environment variables for API key.
- [https://exchangeratesapi.io] — (API used)

## 📦 Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/RahilJain1366/Currency_Convertor.git
cd Currency_Convertor
cargo build
cargo run
```

For the free version, only EUR can be used as the base currency

Enter base currency: USD
Enter target currency: EUR
Enter amount: 100
100 USD = 91.23 EUR

