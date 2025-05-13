# 🧮 pricing_kit

A flexible and extensible Rust library for calculating and managing product pricing, markups, commissions, and currency conversions.

## ✨ Features

- ✅ Define a **base (buy) price** and calculate **sell prices** based on:
  - ➕ Flat amount markups
  - 📈 Percentage markups
  - 💰 Commissions in other currencies
- 🌐 Support for **multi-currency** operations using exchange rates
- 💸 Support for **adjustments** such as:
  - 🧾 **Tax** calculations with percentage-based rates
  - 💸 **Discounts** based on percentage
  - 🏷️ **Fixed fees** with customizable currencies
- 🧱 Clean and extensible API design, ready for future enhancements


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

## ⚡ Adjustment feature example

```rust
use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType};
use pricing_kit::PriceAdjustment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    let mut pricing = PricingDetail::new(100.0, usd.clone(), idr.clone());
    pricing.set_markup(MarkupType::Percentage(20.0));
    pricing.apply_markup(&converter);

    let adjustments = vec![
        PriceAdjustment::Tax {
            name: "Tax 11%".into(),
            percentage: 11.0,
        },
        PriceAdjustment::Discount {
            name: "Discount".into(),
            percentage: 5.0,
        },
        PriceAdjustment::Fixed {
            name: "Promo New Year".to_string(),
            amount: 10.0,
            currency: pricing.sell_currency.clone(),
        }
    ];

    pricing.apply_adjustments(&adjustments, &converter);

    let json = serde_json::to_string_pretty(&pricing)?;
    println!("==================\nAdjustment Pricing:\n{}", json);

    Ok(())
}
```

## 🎯 Crate Goals

This crate is designed to be:

- 👶 Easy to use for common e-commerce and fintech pricing scenarios
- 🧮 Accurate, currency-aware, and reliable in financial calculations
- 🔌 Ready for dynamic adjustments such as tax, discount, and fixed fees
- 🧱 Extensible for future features like tiered pricing, tax rules, and promotions

---

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

