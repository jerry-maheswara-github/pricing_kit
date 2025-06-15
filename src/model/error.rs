use thiserror::Error;

/// Represents possible errors that can occur during currency conversion operations.
///
/// These errors typically indicate issues when trying to retrieve exchange rates
/// or perform calculations involving currency values.
#[derive(Debug, Error)]
pub enum CurrencyConverterError {
    /// An error indicating that an exchange rate for a specific currency
    /// could not be found within the `CurrencyConverter`.
    ///
    /// The contained `String` provides the code of the currency for which
    /// the rate was not found.
    #[error("Exchange rate not found for currency: {0}")]
    RateNotFound(String),

    /// An error indicating that a division by zero occurred during a currency
    /// conversion calculation. This typically happens if an exchange rate
    /// used as a divisor is zero, which is an invalid state for currency rates.
    #[error("Division by zero occurred during conversion.")]
    DivisionByZero,
}

/// Represents possible errors that can occur specifically during pricing calculations
/// within the `PricingDetail` and related modules.
///
/// This enum wraps more granular errors (like `CurrencyConverterError`)
/// to provide context on where in the pricing pipeline the error occurred.
#[derive(Debug, Error)]
pub enum PricingError {
    /// An error indicating that a currency rate calculation failed.
    ///
    /// This variant wraps a `CurrencyConverterError`, providing more context
    /// that the conversion failure happened specifically during the initial
    /// rate calculation phase (e.g., when determining `buy_currency_rate`
    /// or `sell_currency_rate`).
    #[error("Currency rate calculation failed: {0}")]
    RateCalculationFailed(CurrencyConverterError),

    /// An error indicating that a markup calculation resulted in an invalid state.
    ///
    /// This typically occurs when markup parameters are logically impossible
    /// or lead to undefined financial results (e.g., a commission percentage
    /// of 100% or more, which would cause infinite price calculation).
    /// The contained `String` provides a detailed error message.
    #[error("Invalid markup calculation: {0}")]
    InvalidMarkupCalculation(String),

    /// An error indicating that an adjustment (like tax, discount, or fixed fee)
    /// failed during its application to the price.
    ///
    /// This variant wraps a `CurrencyConverterError`, suggesting that the
    /// adjustment failure was due to an underlying currency conversion issue
    /// (e.g., converting a fixed adjustment amount from one currency to another).
    #[error("Price adjustment failed: {0}")]
    AdjustmentFailed(CurrencyConverterError),
}