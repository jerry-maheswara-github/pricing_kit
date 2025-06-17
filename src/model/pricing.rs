use serde::{Deserialize, Serialize};
use crate::model::adjustment::{AdjustmentKind, AppliedAdjustment, PriceAdjustment};
use crate::model::currency::{Currency, CurrencyConverter, CurrencyConverterError};
use crate::model::markup::MarkupType;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::PricingError;

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
///   The final price after applying the markup and adjustments, in the `sell_currency`.
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
/// - `applied_adjustments`:
///   A list of adjustments (e.g., discounts, fees) applied to the pricing calculation.
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
    pub buy_price: Decimal,
    pub sell_price: Decimal, // This will be the final price after markup & adjustment.
    pub buy_currency: Currency,
    pub sell_currency: Currency,
    pub markup: Option<MarkupType>,
    pub markup_value_in_buy_currency: Option<Decimal>,
    pub markup_value_in_sell_currency: Option<Decimal>,
    pub converted_buy_price: Option<Decimal>, // buy_price + markup_in_buy_currency
    pub buy_currency_rate: Option<Decimal>,
    pub sell_currency_rate: Option<Decimal>,
    pub exchange_rate: Option<Decimal>,
    pub applied_adjustments: Vec<AppliedAdjustment>,
}

impl PricingDetail {
    /// Creates a new `PricingDetail` instance.
    ///
    /// This constructor only initializes the basic pricing parameters.
    /// To calculate the full pricing details including markup and adjustments,
    /// `calculate_final_price` method must be called.
    pub fn new(buy_price: Decimal, buy_currency: Currency, sell_currency: Currency) -> Self {
        Self {
            buy_price,
            sell_price: dec!(0.0), 
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

    /// Calculates and applies markup to the buy price.
    ///
    /// This method updates internal fields related to markup and currency rates.
    /// It must be called before `apply_adjustments`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all required exchange rates are found and markup calculation is successful.
    /// `Err(PricingError)` otherwise.
    pub fn apply_markup(&mut self, converter: &CurrencyConverter) -> Result<(), PricingError> {
        // --- 1. Retrieve Exchange Rates Safely (Using Result)---
        let buy_rate = converter.get_exchange_rate(&self.buy_currency)
            .ok_or_else(|| CurrencyConverterError::RateNotFound(self.buy_currency.get_code().to_string()))
            .map_err(PricingError::RateCalculationFailed)?;

        let sell_rate = converter.get_exchange_rate(&self.sell_currency)
            .ok_or_else(|| CurrencyConverterError::RateNotFound(self.sell_currency.get_code().to_string()))
            .map_err(PricingError::RateCalculationFailed)?;

        if buy_rate.is_zero() {
            return Err(PricingError::RateCalculationFailed(CurrencyConverterError::DivisionByZero));
        }
        let exchange_rate = sell_rate / buy_rate;

        self.buy_currency_rate = Some(buy_rate);
        self.sell_currency_rate = Some(sell_rate);
        self.exchange_rate = Some(exchange_rate);

        // --- 2. Handle Markup Calculation ---
        let markup_in_buy = match &self.markup {
            Some(MarkupType::Amount { value, currency }) => {
                converter.convert(*value, currency, &self.buy_currency)
                    .map_err(PricingError::RateCalculationFailed)?
            }
            Some(MarkupType::Percentage(pct)) => {
                self.buy_price * (*pct / dec!(100.0))
            }
            Some(MarkupType::Commission(pct)) => {
                if *pct >= dec!(100.0) {
                    return Err(PricingError::InvalidMarkupCalculation(
                        format!("Commission percentage ({}) must be less than 100.", pct)
                    ));
                }
                self.buy_price * (*pct / (dec!(100.0) - pct))
            }
            None => dec!(0.0),
        };

        self.markup_value_in_buy_currency = Some(markup_in_buy);
        let sell_base = self.buy_price + markup_in_buy;
        self.converted_buy_price = Some(sell_base);
        let initial_sell_price = sell_base * exchange_rate;
        self.markup_value_in_sell_currency = Some(markup_in_buy * exchange_rate);
        self.sell_price = initial_sell_price;

        Ok(())
    }

    /// Applies a list of price adjustments (taxes, discounts, fixed fees) to the sell price.
    ///
    /// This method modifies the `sell_price` based on the given adjustments
    /// and populates the `applied_adjustments` list.
    /// This method should typically be called after `apply_markup`.
    ///
    /// # Arguments
    ///
    /// * `adjustments` - A slice of `PriceAdjustment` to apply.
    /// * `converter` - A reference to the `CurrencyConverter`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all adjustments are applied successfully.
    /// `Err(PricingError)` if any currency conversion during adjustment fails.
    pub fn apply_adjustments(
        &mut self,
        adjustments: &[PriceAdjustment],
        converter: &CurrencyConverter,
    ) -> Result<(), PricingError> {
        let mut current_sell_price = self.sell_price;
        self.applied_adjustments.clear();

        for adj in adjustments {
            let applied = match adj {
                PriceAdjustment::Tax { name, percentage } => {
                    let amt = current_sell_price * (*percentage / dec!(100.0));
                    current_sell_price += amt;
                    AppliedAdjustment {
                        kind: AdjustmentKind::Tax,
                        name: name.clone(),
                        percentage: Some(*percentage),
                        original_currency: Some(self.sell_currency.clone()),
                        original_amount: None,
                        applied_amount: amt,
                    }
                }

                PriceAdjustment::Discount { name, percentage } => {
                    let amt = current_sell_price * (*percentage / dec!(100.0));
                    current_sell_price -= amt;
                    AppliedAdjustment {
                        kind: AdjustmentKind::Discount,
                        name: name.clone(),
                        percentage: Some(*percentage),
                        original_currency: Some(self.sell_currency.clone()),
                        original_amount: None,
                        applied_amount: -amt,
                    }
                }

                PriceAdjustment::Fixed { name, amount, currency } => {
                    let converted_amount_in_sell_currency = converter.convert(*amount, currency, &self.sell_currency)
                        .map_err(PricingError::AdjustmentFailed)?;

                    current_sell_price += converted_amount_in_sell_currency;
                    AppliedAdjustment {
                        kind: AdjustmentKind::Fixed,
                        name: name.clone(),
                        percentage: None,
                        original_currency: Some(currency.clone()),
                        original_amount: Some(*amount),
                        applied_amount: converted_amount_in_sell_currency,
                    }
                }
            };
            self.applied_adjustments.push(applied);
        }

        self.sell_price = current_sell_price;
        Ok(())
    }

    /// Recalculates all pricing details from scratch, applying markup and adjustments.
    ///
    /// This is the primary method to ensure all derived pricing fields are up-to-date
    /// after initial creation or modification of `buy_price`, `currencies`, `markup`,
    /// or `adjustments`.
    ///
    /// # Arguments
    ///
    /// * `converter` - A reference to the `CurrencyConverter`.
    /// * `adjustments` - A slice of `PriceAdjustment` to apply after markup.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all calculations and applications are successful.
    /// `Err(PricingError)` if any underlying calculation or conversion fails.
    pub fn calculate_final_price(
        &mut self,
        converter: &CurrencyConverter,
        adjustments: &[PriceAdjustment],
    ) -> Result<(), PricingError> {
        self.apply_markup(converter)?;
        self.apply_adjustments(adjustments, converter)?;
        Ok(())
    }
}