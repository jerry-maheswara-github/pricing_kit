use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::model::currency::Currency;

/// Represents different types of markup that can be applied to a product's price.
///
/// Markup is the added value on top of the base (buy) price, and can be represented in
/// several formats:
///
/// - A fixed amount in a specific currency
/// - A percentage increase relative to the buy price
/// - A commission-style markup, where the markup is calculated based on the final price
///
/// # Variants
///
/// - `Amount`:
///   A fixed value of markup in a specific currency (e.g. 50,000 IDR or 10 USD).
///   The system will convert this amount to the `buy_currency` internally.
///
/// - `Percentage`:
///   A simple percentage markup added to the buy price (e.g. 10% → buy_price × 1.10).
///
/// - `Commission`:
///   A commission-style markup where the final sell price should include a certain
///   percentage as commission. Internally calculated as:
///   `buy_price / (1 - commission%)`
///
/// # Examples
///
/// ```code
/// MarkupType::Amount { value: 50000.0, currency: Currency::new("IDR", "Indonesian Rupiah") };
/// MarkupType::Percentage(15.0);
/// MarkupType::Commission(10.0);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "markup_type", rename_all = "snake_case")]
pub enum MarkupType {
    /// A fixed markup amount in a specific currency.
    Amount {
        /// The fixed markup value (e.g. 50_000.0).
        value: Decimal,
        /// The currency in which the markup value is denominated.
        currency: Currency,
    },
    /// A markup defined as a percentage of the buy price.
    Percentage(Decimal),
    /// A commission-based markup (markup as a percentage of the final price).
    Commission(Decimal),
}
