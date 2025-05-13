# 🧮 pricing_kit

A flexible and extensible Rust library for calculating and managing product pricing, markups, commissions, and currency conversions.

## ✨ Features

- ✅ Define a **base (buy) price** and calculate **sell prices** based on:
  - ➕ Flat amount markups
  - 📈 Percentage markups
  - 💰 Commissions in other currencies
- 🌐 Support for **multi-currency** operations using exchange rates
- 🧱 Clean and extensible API design

## ⚡ Quick Start

```rust
use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    let mut pricing = PricingDetail::new(100.0, usd.clone(), idr.clone());
    pricing.set_markup(MarkupType::Amount {
        value: 3500.0,
        currency: idr.clone(),
    });

    pricing.apply_markup(&converter);

    let json = serde_json::to_string_pretty(&pricing)?;
    println!("Pricing:\n{}", json);

    Ok(())
}
```

## 🎯 Crate Goals

This crate is designed to be:

- 👶 Easy to use for common e-commerce and fintech pricing scenarios
- 🧮 Accurate and currency-aware
- 🔌 Ready for expansion with tax, discount, or tiered pricing modules in the future

## 📖 License

This project is licensed under the Apache-2.0 license. [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)

---

## 🧑 Author
Jerry Maheswara <jerrymaheswara@gmail.com>

---

## ❤️ Built with Love in Rust

This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.

---

## 👋 Contributing

Pull requests, issues, and feedback are welcome!
If you find this crate useful, give it a ⭐ and share it with others in the Rust community.

---

