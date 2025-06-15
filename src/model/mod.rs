/// Currency-related definitions and helpers, such as currency codes and exchange rates.
///
/// This module contains:
/// - `Currency`: Struct representing a currency (e.g., USD, IDR).
/// - `CurrencyConverter`: Utility for managing and converting exchange rates between currencies.
pub mod currency;

/// Core pricing logic, including buying/selling prices, currency conversion, and pricing details.
///
/// This module provides:
/// - `PricingDetail`: Main struct for managing price calculation flows.
/// - Support for applying markup and calculating final selling price.
pub mod pricing;

/// Markup strategies used in pricing, including fixed amount, percentage, and commission models.
///
/// This module includes:
/// - `MarkupType`: Enum representing different markup mechanisms.
/// - Logic to calculate markup relative to the buy price and currencies.
pub mod markup;

/// Price adjustments such as tax, discount, and fixed fees that affect final selling price.
///
/// This module includes:
/// - `PriceAdjustment`: Enum representing raw, definable adjustments.
/// - `AppliedAdjustment`: Struct for finalized adjustment values after currency conversion.
/// - Application logic for transforming `PriceAdjustment` into `AppliedAdjustment`.
pub mod adjustment;

/// Represents possible errors that can occur during currency conversion.
pub mod error;

pub use currency::*;
pub use pricing::*;
pub use markup::*;
pub use adjustment::*;
pub use error::*;