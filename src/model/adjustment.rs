use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::Currency;

/// Represents additional price modifications such as tax, discount, or fixed fees.
///
/// This enum is used to define adjustments that can be applied to a `PricingDetail`.
/// These adjustments are later converted and applied using a `CurrencyConverter`,
/// resulting in one or more `AppliedAdjustment` entries with actual values.
///
/// # Variants
///
/// - `Tax`:
///   Represents a tax to be applied as a percentage of the current sell price.
///   Useful for applying VAT, Tax, or sales tax.
///
///   - `name`: A human-readable label (e.g., `"Tax 11%"`)
///   - `percentage`: The tax rate as a percentage (e.g., `11.0` for 11%)
///
/// - `Discount`:
///   Represents a discount applied as a percentage of the current sell price.
///   Useful for promotions or campaigns.
///
///   - `name`: A human-readable label (e.g., `"Promo New Year"`)
///   - `percentage`: The discount rate as a percentage (e.g., `5.0` for 5%)
///
/// - `Fixed`:
///   Represents a fixed fee adjustment, such as a service or admin fee.
///   This amount can be in a different currency and will be converted accordingly.
/// 
///   Use `sell_currency` as the default currency for fixed amount adjustments.
///   Caller is responsible for providing the correct currency context.
///
///   - `name`: A human-readable label (e.g., `"Admin Fee"`)
///   - `amount`: The raw fixed amount before conversion
///   - `currency`: The original currency of the fixed amount
///
/// # Example
///
/// ```rust
/// use rust_decimal_macros::dec;
/// use pricing_kit::{Currency, PriceAdjustment};
/// let discount = PriceAdjustment::Discount {
///     name: "Year End Promo".into(),
///     percentage: dec!(5.0),
/// };
///
/// let tax = PriceAdjustment::Tax {
///     name: "Tax 11%".into(),
///     percentage: dec!(11.0),
/// };
///
/// let fixed_fee = PriceAdjustment::Fixed {
///     name: "Admin Fee".into(),
///     amount: dec!(2.0),
///     currency: Currency::new("USD", "US Dollar"),
/// };
/// ```
///
/// Adjustments are intended to be applied in order using `PricingDetail::apply_adjustments()`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "price_adjustment", rename_all = "snake_case")]
pub enum PriceAdjustment {
    Tax {
        name: String,
        percentage: Decimal,
    },
    Discount {
        name: String,
        percentage: Decimal,
    },
    Fixed {
        name: String,
        amount: Decimal,
        currency: Currency,
    },
}

/// Represents a final, applied price adjustment (e.g., tax, discount, or fixed fee)
/// that has been calculated and converted to the target sell currency.
///
/// This structure is typically the result of applying a `PriceAdjustment`
/// to a `PricingDetail` after performing all necessary conversions.
///
/// # Fields
///
/// - `kind`: A string label indicating the type of adjustment: `"Tax"`, `"Discount"`, or `"Fixed"`.
/// - `name`: A human-readable name of the adjustment (e.g., `"Tax 11%"`, `"Promo New Year"`).
/// - `percentage`: Optional percentage value used for `"Tax"` and `"Discount"` adjustments.
/// - `original_currency`: Currency in which the original adjustment was defined, if applicable (usually only for fixed adjustments).
/// - `original_amount`: Original amount of the adjustment before conversion, if applicable.
/// - `applied_amount`: The final amount after calculation and conversion, always expressed in the target `sell_currency`.
///
/// # Example (JSON)
///
/// ```json
/// {
///   "kind": "Tax",
///   "name": "Tax 11%",
///   "percentage": 11.0,
///   "original_currency": {
///     "code": "IDR",
///     "name": "Indonesian Rupiah"
///   },
///   "original_amount": 1996500.0,
///   "applied_amount": 1996500.0
/// }
/// ```
///
/// This struct is intended to be serialization-friendly for logs, APIs, and reporting.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "applied_adjustment", rename_all = "snake_case")]
pub struct AppliedAdjustment {
    /// "Tax", "Discount", "Fixed"
    pub kind: AdjustmentKind,

    /// e.g., "Tax 11%"
    pub name: String, 

    /// for Tax/Discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Decimal>,

    /// always sell_currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_currency: Option<Currency>,

    /// for Fixed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<Decimal>,

    /// always in sell_currency
    pub applied_amount: Decimal,
}

/// Defines the category or type of price adjustment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "adjustment_kind", rename_all = "snake_case")]
pub enum AdjustmentKind {
    Tax,
    Discount,
    Fixed,
}