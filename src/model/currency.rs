use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
pub(crate) use crate::CurrencyConverterError;

/// Represents a currency with a standard code and a human-readable name.
///
/// This struct is commonly used to denote monetary units in pricing,
/// conversions, and transactions. It follows the ISO 4217 currency code format
/// (e.g. "USD" for US Dollar, "IDR" for Indonesian Rupiah).
///
/// # Fields
///
/// - `code`:
///   A 3-letter ISO currency code in uppercase (e.g. `"USD"`, `"EUR"`, `"IDR"`).
///
/// - `name`:
///   The full name of the currency (e.g. `"US Dollar"`, `"Indonesian Rupiah"`).
///
/// # Traits
///
/// - `Serialize`, `Deserialize`: Supports JSON and other serialization formats (via `serde`)
/// - `Debug`: For logging and debugging
/// - `Clone`, `PartialEq`, `Eq`, `Hash`: Enables usage in maps, sets, and comparisons
///
/// # Example
///
/// ```
/// use pricing_kit::Currency;
/// let usd = Currency::new("USD", "US Dollar");
/// assert_eq!(usd.get_code(), "USD");
/// assert_eq!(usd.get_name(), "US Dollar");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    code: String,
    name: String,
}

impl Currency {
    /// Creates a new `Currency` instance with the given code and name.
    ///
    /// # Arguments
    ///
    /// * `code` - The currency code (e.g., "USD", "IDR").
    /// * `name` - The currency name (e.g., "American Dollar", "Indonesian Rupiah").
    ///
    /// # Returns
    ///
    /// A `Currency` struct initialized with the given code and name.
    pub fn new(code: &str, name: &str) -> Self {
        Currency {
            code: code.to_string(),
            name: name.to_string(),
        }
    }

    /// Returns the code of the currency (e.g., "USD").
    pub fn get_code(&self) -> &str {
        &self.code
    }

    /// Returns the name of the currency (e.g., "American Dollar").
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// A simple currency conversion utility that stores exchange rates
/// and performs conversions between different currencies.
///
/// `CurrencyConverter` maintains a mapping between currency codes (e.g., `"USD"`, `"IDR"`)
/// and their corresponding exchange rates relative to a common base (typically 1.0 for the base currency).
///
/// The converter assumes linear conversion using the formula:
///
/// ```text
/// amount_in_target = (amount / rate_from) * rate_to
/// ```
///
/// # Fields
///
/// - `exchange_rates`:
///   A map of currency codes (`String`) to their exchange rate values (`Decimal`).
///   These rates are relative to an arbitrary common base.
///
/// # Example
///
/// ```
/// # use pricing_kit::{Currency, CurrencyConverter, CurrencyConverterError};
/// # use rust_decimal_macros::dec;
/// let mut converter = CurrencyConverter::new();
/// let usd = Currency::new("USD", "US Dollar");
/// let idr = Currency::new("IDR", "Indonesian Rupiah");
///
/// converter.add_exchange_rate(&usd, dec!(1.0));       // base currency
/// converter.add_exchange_rate(&idr, dec!(16500.0));   // 1 USD = 16500 IDR
///
/// let amount_in_idr = converter.convert(dec!(100.0), &usd, &idr).unwrap();
/// assert_eq!(amount_in_idr, dec!(1_650_000.0));
/// ```
///
/// # Usage Notes
///
/// - Missing exchange rates will result in an error (`CurrencyConverterError::RateNotFound`),
///   requiring explicit handling.
/// - To get precise results, make sure exchange rates are consistently set relative to the same base currency.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrencyConverter {
    exchange_rates: HashMap<String, Decimal>,
}

impl CurrencyConverter {
    /// Creates a new `CurrencyConverter` instance.
    ///
    /// # Returns
    ///
    /// A new `CurrencyConverter` with no exchange rates set.
    pub fn new() -> Self {
        CurrencyConverter {
            exchange_rates: HashMap::new(),
        }
    }

    /// Adds an exchange rate for a specific currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency to add the exchange rate for.
    /// * `rate` - The exchange rate for the given currency (as `Decimal`).
    pub fn add_exchange_rate(&mut self, currency: &Currency, rate: Decimal) {
        self.exchange_rates.insert(currency.get_code().to_string(), rate);
    }

    /// Converts an amount from one currency to another using the exchange rates.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to convert (as `Decimal`).
    /// * `from` - The currency to convert from.
    /// * `to` - The currency to convert to.
    ///
    /// # Returns
    ///
    /// `Ok(Decimal)` if the conversion is successful, or `Err(CurrencyConverterError)`
    /// if an exchange rate is missing or a division by zero occurs.
    pub fn convert(&self, amount: Decimal, from: &Currency, to: &Currency) -> Result<Decimal, CurrencyConverterError> {
        if from.get_code() == to.get_code() {
            return Ok(amount);
        }

        let from_rate = self.exchange_rates.get(from.get_code())
            .ok_or_else(|| CurrencyConverterError::RateNotFound(from.get_code().to_string()))?;

        let to_rate = self.exchange_rates.get(to.get_code())
            .ok_or_else(|| CurrencyConverterError::RateNotFound(to.get_code().to_string()))?;

        // Check division by zero before performing the operation.
        if from_rate.is_zero() {
            return Err(CurrencyConverterError::DivisionByZero);
        }

        // Decimal automatically handles precision
        let converted_amount = (amount / from_rate) * to_rate;

        Ok(converted_amount)
    }

    /// Retrieves the exchange rate for the specified currency from the stored exchange rates.
    ///
    /// This function looks up the exchange rate for a given currency code (e.g., `"USD"`, `"IDR"`)
    /// and returns the rate as a `Decimal` value, representing the amount of the base currency equivalent
    /// to one unit of the given currency. If the exchange rate is not found, it returns `None`.
    ///
    /// # Arguments
    ///
    /// - `currency`:
    ///   A reference to the `Currency` for which the exchange rate is needed.
    ///
    /// # Returns
    ///
    /// - `Option<Decimal>`:
    ///   - `Some(Decimal)` if the exchange rate exists for the given currency code.
    ///   - `None` if the currency code does not have a stored exchange rate.
    ///
    /// # Example
    ///
    /// ```
    /// # use pricing_kit::{Currency, CurrencyConverter};
    /// # use rust_decimal_macros::dec;
    /// let mut converter = CurrencyConverter::new();
    /// let usd = Currency::new("USD", "US Dollar");
    /// let idr = Currency::new("IDR", "Indonesian Rupiah");
    ///
    /// // Adding exchange rates
    /// converter.add_exchange_rate(&usd, dec!(1.0));      // Base currency
    /// converter.add_exchange_rate(&idr, dec!(16500.0));  // 1 USD = 16,500 IDR
    ///
    /// // Get exchange rate for USD
    /// let usd_rate = converter.get_exchange_rate(&usd);
    /// assert_eq!(usd_rate, Some(dec!(1.0)));  // USD rate is 1.0 (base currency)
    ///
    /// // Get exchange rate for IDR
    /// let idr_rate = converter.get_exchange_rate(&idr);
    /// assert_eq!(idr_rate, Some(dec!(16500.0)));  // IDR rate is 16500
    /// ```
    ///
    /// # Notes
    ///
    /// - If the currency is not found in the exchange rates map, the method returns `None`.
    /// - The base currency (often `USD` or any standard reference) should be initialized with a rate of `1.0`.
    pub fn get_exchange_rate(&self, currency: &Currency) -> Option<Decimal> {
        self.exchange_rates.get(currency.get_code()).copied()
    }
}