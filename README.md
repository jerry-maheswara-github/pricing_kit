# 🧮 pricing_kit

A flexible and extensible Rust library for calculating and managing product pricing, markups, commissions, and currency conversions.

## ✨ Features

- Define a **base (buy) price** and calculate **sell prices** based on:
  - Flat amount markups
  - Percentage markups
  - Commissions in other currencies
- Support for **multi-currency** operations using exchange rates, ensuring **financial precision** with `Decimal` types.
- Support for **adjustments** such as:
  - **Tax** calculations with percentage-based rates
  - **Discounts** based on percentage
  - **Fixed fees** with customizable currencies
- Clean and extensible API design, ready for future enhancements.

---

## ⚡ Quick Start

```rust
use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType, dec, ToPrimitive};

fn main() {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, dec!(1.0));
    converter.add_exchange_rate(&idr, dec!(16500.0));

    let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());

    pricing.markup = Some(MarkupType::Amount {
        value: dec!(3500),
        currency: idr.clone(),
    });

    pricing.apply_markup(&converter).expect("Failed to apply markup");

    println!("Pricing after markup:\n{:#?}", pricing);

    // Manual calculation for comparative purpose:
    // 1. Buy price: 100.0 USD
    // 2. Markup: Amount 3500 IDR → converted to USD = 3500 / 16500 = ~0.21212121 USD
    // 3. Converted buy price: 100 + 0.21212121 = ~100.21212121 USD
    // 4. Exchange rate USD → IDR = 16500
    // 5. Final sell price: 100.21212121 * 16500 = 1_653_500.0 IDR

    if let Some(total_f64) = pricing.sell_price.to_f64() {
        println!("Total sell price as f64: {}", total_f64);
    }
}
```


---

## ⚡ Adjustment Feature Example

```rust
use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType, dec, PriceAdjustment, ToPrimitive};

fn main() {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, dec!(1.0));
    converter.add_exchange_rate(&idr, dec!(16500.0));

    let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());
    pricing.markup = Some(MarkupType::Percentage(dec!(20.0)));
    pricing.apply_markup(&converter).expect("Failed to apply markup");

    let adjustments = vec![
        PriceAdjustment::Tax {
            name: "Tax 11%".into(),
            percentage: dec!(11.0),
        },
        PriceAdjustment::Discount {
            name: "Discount".into(),
            percentage: dec!(5.0),
        },
        PriceAdjustment::Fixed {
            name: "Promo New Year".to_string(),
            amount: dec!(10.0),
            currency: pricing.sell_currency.clone(),
        }
    ];

    pricing.apply_adjustments(&adjustments, &converter).expect("Failed to apply adjustments");

    println!("Adjustment Pricing:\n{:#?}", pricing);

    // Manual calculation for comparative purpose:
    // 1. Buy price in USD: 100.0
    // 2. Markup 20% -> 100 * 0.2 = 20 USD
    // 3. Converted buy price: 100 + 20 = 120 USD
    // 4. Sell price in IDR: 120 * 16500 = 1_980_000 IDR

    // Adjustments:
    // + Tax 11% of 1_980_000 = 217_800 -> 2_197_800
    // - Discount 5% of 2_197_800 = 109_890 -> 2_087_910
    // + Fixed amount 10 IDR -> final = 2_087_920.0 IDR

    if let Some(total_f64) = pricing.sell_price.to_f64() {
        println!("Total sell price as f64: {}", total_f64);
    }
}
```

---

## 🎯 Crate Goals

This crate is designed to be:

- Easy to use for common e-commerce and fintech pricing scenarios.
- **Accurate**, currency-aware, and reliable in financial calculations through explicit `Decimal` usage.
- Ready for dynamic adjustments such as tax, discount, and fixed fees.
- Extensible for future features like tiered pricing, tax rules, and promotions.

---

## ♻️ Re-exports for Convenience

For ease of use and to ensure robust financial calculations, `pricing_kit` **re-exports** the following types and macros from the `rust_decimal` ecosystem:

-   `Decimal` (from the `rust_decimal` crate): Used for all currency amounts and exchange rates to ensure arbitrary precision arithmetic and avoid floating-point inaccuracies.
-   `dec!` macro (from the `rust_decimal_macros` crate): A convenient macro for creating `Decimal` literals in your code.

This means you can use `pricing_kit::Decimal` and `pricing_kit::dec!` directly without needing to explicitly import them from their original crates, although you still need to declare `rust_decimal` and `rust_decimal_macros` in your `Cargo.toml`.

```toml
[dependencies]
rust_decimal = "1.37.2"
rust_decimal_macros = "1.37.1"
```
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
