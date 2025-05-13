use serde::{Deserialize, Serialize};
use crate::model::currency::{Currency, CurrencyConverter};
use crate::model::markup::MarkupType;

/// Represents the full pricing information of a product, including
/// markup and currency conversion details.
///
/// This struct is designed to track both the original purchase price
/// and the final selling price, supporting different currencies for
/// buy and sell. It also tracks all intermediate calculations for
/// transparency, auditability, and debugging.
///
/// # Fields
///
/// - `buy_price`:
///   The original price of the product in the `buy_currency`.
///
/// - `sell_price`:
///   The final price after applying the markup, in the `sell_currency`.
///
/// - `buy_currency`:
///   The currency used when purchasing the product (e.g., USD).
///
/// - `sell_currency`:
///   The currency used for selling the product (e.g., IDR).
///
/// - `markup`:
///   The markup applied to the product, which can be a fixed amount,
///   a percentage, or a commission-style increase.
///
/// - `markup_value_in_buy_currency`:
///   The computed markup value after converting (if needed) to the buy currency.
///
/// - `markup_value_in_sell_currency`:
///   The markup amount represented in the sell currency.
///
/// - `converted_buy_price`:
///   The buy price after markup but before converting to `sell_currency`.
///
/// - `buy_currency_rate`:
///   The exchange rate of `buy_currency` relative to a base (typically 1.0 for base currency).
///
/// - `sell_currency_rate`:
///   The exchange rate of `sell_currency` relative to the same base.
///
/// - `exchange_rate`:
///   Derived from `sell_currency_rate / buy_currency_rate`; represents the effective
///   rate from `buy_currency` to `sell_currency`.
///
/// # Example Use Case
///
/// A product is bought in USD, marked up using an IDR amount, and sold in IDR.
/// This struct captures all relevant price states and conversions.
///
/// # Serialization
///
/// Supports serialization with `serde` for logging, debugging, or API responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct PricingDetail {
    /// The base price at which the product was bought.
    pub buy_price: f64,
    /// The final price at which the product will be sold.
    pub sell_price: f64,
    /// The currency used for the buy transaction.
    pub buy_currency: Currency,
    /// The currency used for the sell transaction.
    pub sell_currency: Currency,
    /// The markup strategy applied to the buy price.
    pub markup: Option<MarkupType>,

    /// The markup value converted to the buy currency.
    pub markup_value_in_buy_currency: Option<f64>,
    /// The markup value represented in the sell currency.
    pub markup_value_in_sell_currency: Option<f64>,
    /// The buy price after markup, before currency conversion.
    pub converted_buy_price: Option<f64>,

    /// Exchange rate of the buy currency relative to a common base.
    pub buy_currency_rate: Option<f64>,
    /// Exchange rate of the sell currency relative to the same base.
    pub sell_currency_rate: Option<f64>,
    /// Effective exchange rate from buy_currency to sell_currency.
    pub exchange_rate: Option<f64>,
}

impl PricingDetail {

    pub fn new(buy_price: f64, buy_currency: Currency, sell_currency: Currency) -> Self {
        Self {
            buy_price,
            sell_price: 0.0,
            buy_currency,
            sell_currency,
            markup: None,
            markup_value_in_buy_currency: None,
            markup_value_in_sell_currency: None,
            converted_buy_price: None,
            buy_currency_rate: None,
            sell_currency_rate: None,
            exchange_rate: None,
        }
    }

    pub fn get_buy_price(&self) -> f64 {
        self.buy_price
    }

    pub fn get_sell_price(&self) -> f64 {
        self.sell_price
    }

    pub fn get_buy_currency(&self) -> &Currency {
        &self.buy_currency
    }

    pub fn get_sell_currency(&self) -> &Currency {
        &self.sell_currency
    }

    pub fn get_markup(&self) -> &Option<MarkupType> {
        &self.markup
    }

    pub fn set_markup(&mut self, markup: MarkupType) {
        self.markup = Some(markup);
    }

    pub fn get_buy_currency_rate(&self) -> Option<f64> {
        self.buy_currency_rate
    }

    pub fn get_sell_currency_rate(&self) -> Option<f64> {
        self.sell_currency_rate
    }

    pub fn get_exchange_rate(&self) -> Option<f64> {
        match (self.buy_currency_rate, self.sell_currency_rate) {
            (Some(buy), Some(sell)) => Some(sell / buy),
            _ => None,
        }
    }

    pub fn get_markup_value_in_buy_currency(&self) -> Option<f64> {
        self.markup_value_in_buy_currency
    }

    pub fn get_markup_value_in_sell_currency(&self) -> Option<f64> {
        self.markup_value_in_sell_currency
    }

    pub fn get_converted_buy_price(&self) -> Option<f64> {
        self.converted_buy_price
    }
    pub fn get_markup_in_sell_currency(&self) -> Option<f64> {
        self.markup_value_in_sell_currency
    }

    pub fn get_markup_in_buy_currency(&self) -> Option<f64> {
        self.markup_value_in_buy_currency
    }

    pub fn apply_markup(&mut self, converter: &CurrencyConverter) {
        let buy_rate = converter.get_exchange_rate(&self.buy_currency).unwrap_or(1.0);
        let sell_rate = converter.get_exchange_rate(&self.sell_currency).unwrap_or(1.0);

        self.buy_currency_rate = Some(buy_rate);
        self.sell_currency_rate = Some(sell_rate);
        self.exchange_rate = Some(sell_rate / buy_rate);

        let markup_in_buy = match &self.markup {
            Some(MarkupType::Amount { value, currency }) => {
                converter.convert(*value, currency, &self.buy_currency)
            },
            Some(MarkupType::Percentage(pct)) => self.buy_price * pct / 100.0,
            Some(MarkupType::Commission(pct)) => self.buy_price * pct / (100.0 - pct),
            None => 0.0,
        };

        self.markup_value_in_buy_currency = Some(markup_in_buy);
        let sell_base = self.buy_price + markup_in_buy;
        self.converted_buy_price = Some(sell_base);

        let converted = (sell_base / buy_rate) * sell_rate;
        self.markup_value_in_sell_currency = Some((markup_in_buy / buy_rate) * sell_rate);
        self.sell_price = converted;
    }
}

