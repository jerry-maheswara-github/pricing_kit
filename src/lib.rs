//! # 🧮 Pricing Kit
//!
//! `pricing_kit` is a flexible and extensible Rust library for calculating and managing  
//! product pricing, markups, commissions, and currency conversions. 💸🌍
//!
//! ## ✨ Features
//!
//! - ✅ Define a **base (buy) price** and calculate **sell prices** based on:
//!   - ➕ Flat amount markups
//!   - 📈 Percentage markups
//!   - 💰 Commissions in other currencies
//! - 🌐 Support for **multi-currency** operations using exchange rates
//! - 🧱 Clean and extensible API design
//!
//! ## ⚡ Quick Start
//!
//! ```rust
//! use pricing_kit::model::Currency;
//! use pricing_kit::model::{CurrencyConverter, Pricing};
//!
//! fn main() {
//!     let usd = Currency::new("USD", "American Dollar");
//!     let idr = Currency::new("IDR", "Indonesian Rupiah");
//!     let mut pricing = Pricing::new(100.0, usd.clone());
//!
//!     let mut converter = CurrencyConverter::new();
//!     converter.add_exchange_rate(&usd, 1.0);
//!     converter.add_exchange_rate(&idr, 14500.0);
//!
//!     pricing.set_sell_price_by_percentage(10.0);
//!     let sell_price_in_idr = pricing.convert_sell_price(&converter, &idr);
//!
//!     println!("💵 Sell price in IDR: {}", sell_price_in_idr);
//! }
//! ```
//!
//! ## 🎯 Crate Goals
//!
//! This crate is designed to be:
//!
//! - 👶 Easy to use for common e-commerce and fintech pricing scenarios
//! - 🧮 Accurate and currency-aware
//! - 🔌 Ready for expansion with tax, discount, or tiered pricing modules in the future
//!
//! ## ⚖️ License
//!
//! Licensed under MIT or Apache-2.0, at your option.
//!
//! ---
//!
//! Made with ❤️ for Rustaceans building products that sell.

pub mod model;