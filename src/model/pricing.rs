use serde::{Deserialize, Serialize};
use crate::model::adjustment::{AppliedAdjustment, PriceAdjustment};
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// A list of adjustments (e.g., discounts, fees) applied to the pricing calculation.
    pub applied_adjustments: Vec<AppliedAdjustment>,
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
            applied_adjustments: vec![],
        }
    }

    pub fn get_buy_price(&self) -> f64 {
        self.buy_price
    }
    
    pub fn set_buy_price(&mut self, value: f64) {
        self.buy_price = value;
    }

    pub fn get_sell_price(&self) -> f64 {
        self.sell_price
    }
    
    pub fn set_sell_price(&mut self, value: f64) {
        self.sell_price = value;
    }

    pub fn get_buy_currency(&self) -> &Currency {
        &self.buy_currency
    }
    
    pub fn set_buy_currency(&mut self, value: Currency) {
        self.buy_currency = value;
    }

    pub fn get_sell_currency(&self) -> &Currency {
        &self.sell_currency
    }
    
    pub fn set_sell_currency(&mut self, value: Currency) {
        self.sell_currency = value;
    }

    pub fn get_markup(&self) -> &Option<MarkupType> {
        &self.markup
    }

    pub fn set_markup(&mut self, markup: Option<MarkupType>) {
        self.markup = markup;
    }

    pub fn get_buy_currency_rate(&self) -> Option<f64> {
        self.buy_currency_rate
    }
    
    pub fn set_buy_currency_rate(&mut self, rate: Option<f64>) {
        self.buy_currency_rate = rate;
    }

    pub fn get_sell_currency_rate(&self) -> Option<f64> {
        self.sell_currency_rate
    }
    
    pub fn set_sell_currency_rate(&mut self, rate: Option<f64>) {
        self.sell_currency_rate = rate;
    }

    pub fn get_exchange_rate(&self) -> Option<f64> {
        match (self.buy_currency_rate, self.sell_currency_rate) {
            (Some(buy), Some(sell)) => Some(sell / buy),
            _ => None,
        }
    }

    pub fn set_exchange_rate(&mut self, value: Option<f64>) {
        self.exchange_rate = value;
    }

    pub fn get_markup_value_in_buy_currency(&self) -> Option<f64> {
        self.markup_value_in_buy_currency
    }

    pub fn set_markup_value_in_buy_currency(&mut self, value: Option<f64>) {
        self.markup_value_in_buy_currency = value;
    }
    
    pub fn get_markup_value_in_sell_currency(&self) -> Option<f64> {
        self.markup_value_in_sell_currency
    }

    pub fn set_markup_value_in_sell_currency(&mut self, value: Option<f64>) {
        self.markup_value_in_sell_currency = value;
    }
    
    pub fn get_converted_buy_price(&self) -> Option<f64> {
        self.converted_buy_price
    }
    
    pub fn set_converted_buy_price(&mut self, price: Option<f64>) {
        self.converted_buy_price = price;
    }

    pub fn apply_markup(&mut self, converter: &CurrencyConverter) {
        // Get exchange rates safely
        let Some(buy_rate) = converter.get_exchange_rate(&self.buy_currency) else {
            return;
        };
        let Some(sell_rate) = converter.get_exchange_rate(&self.sell_currency) else {
            return;
        };

        let exchange_rate = sell_rate / buy_rate;

        self.buy_currency_rate = Some(buy_rate);
        self.sell_currency_rate = Some(sell_rate);
        self.exchange_rate = Some(exchange_rate);

        // Handle markup (can be None)
        let markup_in_buy = match &self.markup {
            Some(MarkupType::Amount { value, currency }) => {
                converter.convert(*value, currency, &self.buy_currency)
            }
            Some(MarkupType::Percentage(pct)) => self.buy_price * pct / 100.0,
            Some(MarkupType::Commission(pct)) => self.buy_price * pct / (100.0 - pct),
            None => 0.0,
        };

        self.markup_value_in_buy_currency = Some(markup_in_buy);

        let sell_base = self.buy_price + markup_in_buy;
        self.converted_buy_price = Some(sell_base);

        let sell_price = sell_base * exchange_rate;
        self.markup_value_in_sell_currency = Some(markup_in_buy * exchange_rate);
        self.sell_price = sell_price;
    }

    pub fn apply_adjustments(
        &mut self,
        adjustments: &[PriceAdjustment],
        converter: &CurrencyConverter,
    ) {
        let mut final_price = self.sell_price;
        self.applied_adjustments.clear();

        for adj in adjustments {
            let applied = match adj {
                PriceAdjustment::Tax { name, percentage } => {
                    let amt = final_price * (percentage / 100.0);
                    final_price += amt;
                    AppliedAdjustment {
                        kind: "Tax".into(),
                        name: name.clone(),
                        percentage: Some(*percentage),
                        original_currency: Some(self.sell_currency.clone()),
                        original_amount: Some(amt),
                        applied_amount: amt,
                    }
                }

                PriceAdjustment::Discount { name, percentage } => {
                    let amt = final_price * (percentage / 100.0);
                    final_price -= amt;
                    AppliedAdjustment {
                        kind: "Discount".into(),
                        name: name.clone(),
                        percentage: Some(*percentage),
                        original_currency: Some(self.sell_currency.clone()),
                        original_amount: Some(amt),
                        applied_amount: -amt,
                    }
                }

                PriceAdjustment::Fixed { name, amount, currency } => {
                    let converted = converter.convert(*amount, currency, &self.sell_currency);
                    final_price += converted;
                    AppliedAdjustment {
                        kind: "Fixed".into(),
                        name: name.clone(),
                        percentage: None,
                        original_currency: Some(currency.clone()),
                        original_amount: Some(*amount),
                        applied_amount: converted,
                    }
                }
            };

            self.applied_adjustments.push(applied);
        }

        self.sell_price = final_price;
    }
}

