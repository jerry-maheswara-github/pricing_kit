//! # 🧮 pricing_kit
//!
//! A flexible and extensible Rust library for calculating and managing product pricing, markups, commissions, and currency conversions.
//!
//! ## ✨ Features
//!
//! - Define a **base (buy) price** and calculate **sell prices** based on:
//!   - Flat amount markups
//!   - Percentage markups
//!   - Commissions in other currencies
//! - Support for **multi-currency** operations using exchange rates, ensuring **financial precision** with `Decimal` types.
//! - Support for **adjustments** such as:
//!   - **Tax** calculations with percentage-based rates
//!   - **Discounts** based on percentage
//!   - **Fixed fees** with customizable currencies
//! - Clean and extensible API design, ready for future enhancements.
//!
//! ---
//!
//! ## ⚡ Quick Start
//!
//! ```rust
//! use pricing_kit::{Currency, CurrencyConverter, MarkupType, PricingDetail, dec};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let usd = Currency::new("USD", "American Dollar");
//!     let idr = Currency::new("IDR", "Indonesian Rupiah");
//!
//!     let mut converter = CurrencyConverter::new();
//!     converter.add_exchange_rate(&usd, dec!(1.0));
//!     converter.add_exchange_rate(&idr, dec!(16500.0));
//!
//!     let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());
//!     pricing.markup = Some(MarkupType::Amount {
//!         value: dec!(3500.0),
//!         currency: idr.clone(),
//!     });
//!
//!     pricing.apply_markup(&converter)?;
//!
//!     println!("Pricing after markup:\n{:#?}", pricing);
//!
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## ⚡ Adjustment Feature Example
//!
//! ```rust
//! use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType, PriceAdjustment, dec};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let usd = Currency::new("USD", "American Dollar");
//!     let idr = Currency::new("IDR", "Indonesian Rupiah");
//!
//!     let mut converter = CurrencyConverter::new();
//!     converter.add_exchange_rate(&usd, dec!(1.0));
//!     converter.add_exchange_rate(&idr, dec!(16500.0));
//!
//!     let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());
//!     pricing.markup = Some(MarkupType::Percentage(dec!(20.0)));
//!     pricing.apply_markup(&converter)?;
//!
//!     let adjustments = vec![
//!         PriceAdjustment::Tax {
//!             name: "Tax 11%".into(),
//!             percentage: dec!(11.0),
//!         },
//!         PriceAdjustment::Discount {
//!             name: "Discount".into(),
//!             percentage: dec!(5.0),
//!         },
//!         PriceAdjustment::Fixed {
//!             name: "Promo New Year".to_string(),
//!             amount: dec!(10.0),
//!             currency: pricing.sell_currency.clone(),
//!         }
//!     ];
//!
//!     pricing.apply_adjustments(&adjustments, &converter)?;
//!
//!     println!("Adjustment Pricing:\n{:#?}", pricing);
//!
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## 🎯 Crate Goals
//!
//! This crate is designed to be:
//!
//! - Easy to use for common e-commerce and fintech pricing scenarios.
//! - **Accurate**, currency-aware, and reliable in financial calculations through explicit `Decimal` usage.
//! - Ready for dynamic adjustments such as tax, discount, and fixed fees.
//! - Extensible for future features like tiered pricing, tax rules, and promotions.
//!
//! ---
//!
//! ## ♻️ Re-exports for Convenience
//!
//! For ease of use and to ensure robust financial calculations, `pricing_kit` **re-exports** the following types and macros from the `rust_decimal` ecosystem:
//!
//! -   `Decimal` (from the `rust_decimal` crate): Used for all currency amounts and exchange rates to ensure arbitrary precision arithmetic and avoid floating-point inaccuracies.
//! -   `dec!` macro (from the `rust_decimal_macros` crate): A convenient macro for creating `Decimal` literals in your code.
//!
//! This means you can use `pricing_kit::Decimal` and `pricing_kit::dec!` directly without needing to explicitly import them from their original crates, although you still need to declare `rust_decimal` and `rust_decimal_macros` in your `Cargo.toml`.
//!
//! ---
//!
//! ## 📖 License
//!
//! This project is licensed under the Apache-2.0 license. [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)
//!
//! ---
//!
//! ## 🧑 Author
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## ❤️ Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## 👋 Contributing
//!
//! Pull requests, issues, and feedback are welcome!
//! If you find this crate useful, give it a ⭐ and share it with others in the Rust community.
//!
//! ---

/// Core data model definitions for pricing operations.
pub mod model;

#[doc(inline)]
pub use rust_decimal::Decimal;
#[doc(inline)]
pub use rust_decimal_macros::dec;

pub use model::currency::*;
pub use model::pricing::*;
pub use model::markup::*;
pub use model::adjustment::*;
pub use model::error::*;