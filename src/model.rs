use std::collections::HashMap;

#[derive(Debug)]
pub struct Pricing {
    pub buy_price: f64,
    pub sell_price: f64,
    pub buy_currency: Currency,
    pub sell_currency: Currency,
}

impl Pricing {
    /// Creates a new `Pricing` instance with a given buy price and currency.
    ///
    /// # Arguments
    ///
    /// * `buy_price` - The price at which the product is purchased.
    /// * `buy_currency` - The currency in which the buy price is set.
    ///
    /// # Returns
    ///
    /// A `Pricing` struct with initial buy and sell prices set to `buy_price`.
    pub fn new(buy_price: f64, buy_currency: Currency) -> Self {
        Pricing {
            buy_price,
            sell_price: buy_price,
            buy_currency: buy_currency.clone(),
            sell_currency: buy_currency.clone(),
        }
    }

    /// Sets the selling price based on a fixed amount in another currency.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to be added to the buy price.
    /// * `converter` - A `CurrencyConverter` to handle currency conversion.
    pub fn set_sell_price_by_amount(&mut self, amount: f64, converter: &CurrencyConverter) {
        let amount_in_usd = converter.convert(amount, &self.sell_currency, &self.buy_currency);
        self.sell_price = self.buy_price + amount_in_usd;
    }

    /// Sets the selling price based on a percentage markup.
    ///
    /// # Arguments
    ///
    /// * `percentage` - The percentage by which to increase the buy price.
    pub fn set_sell_price_by_percentage(&mut self, percentage: f64) {
        self.sell_price = self.buy_price * (1.0 + percentage / 100.0);
    }

    /// Sets the selling price based on a commission value.
    ///
    /// # Arguments
    ///
    /// * `commission` - The commission to be added to the buy price.
    /// * `converter` - A `CurrencyConverter` to handle currency conversion.
    pub fn set_sell_price_by_commission(&mut self, commission: f64, converter: &CurrencyConverter) {
        let commission_in_usd = converter.convert(commission, &self.sell_currency, &self.buy_currency);
        self.sell_price = self.buy_price + commission_in_usd;
    }

    /// Converts the sell price to a different currency.
    ///
    /// # Arguments
    ///
    /// * `converter` - A `CurrencyConverter` to handle the conversion.
    /// * `to_currency` - The target currency for the conversion.
    ///
    /// # Returns
    ///
    /// The sell price converted to the target currency.
    pub fn convert_sell_price(&self, converter: &CurrencyConverter, to_currency: &Currency) -> f64 {
        converter.convert(self.sell_price, &self.sell_currency, to_currency)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub struct CurrencyConverter {
    exchange_rates: HashMap<String, f64>,
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
    /// * `rate` - The exchange rate for the given currency.
    pub fn add_exchange_rate(&mut self, currency: &Currency, rate: f64) {
        self.exchange_rates.insert(currency.get_code().to_string(), rate);
    }

    /// Converts an amount from one currency to another using the exchange rates.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to convert.
    /// * `from` - The currency to convert from.
    /// * `to` - The currency to convert to.
    ///
    /// # Returns
    ///
    /// The converted amount in the target currency.
    pub fn convert(&self, amount: f64, from: &Currency, to: &Currency) -> f64 {
        if from.get_code() == to.get_code() {
            return amount;
        }

        let from_rate = *self.exchange_rates.get(from.get_code()).unwrap_or(&1.0);
        let to_rate = *self.exchange_rates.get(to.get_code()).unwrap_or(&1.0);

        (amount / from_rate) * to_rate
    }
}
