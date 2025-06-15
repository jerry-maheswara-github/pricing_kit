#[cfg(test)]
mod tests {
    use pricing_kit::{Currency, CurrencyConverter, MarkupType, PricingDetail, dec};

    fn setup_converter() -> CurrencyConverter {
        let mut converter = CurrencyConverter::new();
        let usd = Currency::new("USD", "US Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");

        converter.add_exchange_rate(&usd, dec!(1.0));
        converter.add_exchange_rate(&idr, dec!(16500.0));
        converter
    }

    fn currencies() -> (Currency, Currency) {
        (
            Currency::new("USD", "US Dollar"),
            Currency::new("IDR", "Indonesian Rupiah"),
        )
    }

    #[test]
    fn test_markup_amount_in_idr() {
        let (usd, idr) = currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(dec!(1000.0), usd.clone(), idr.clone());
        pricing.markup = Some(MarkupType::Amount {
            value: dec!(49500.0),
            currency: idr.clone(),
        });

        // Call apply_markup and unwrap the result (since we expect this to pass in the test)
        pricing.apply_markup(&converter).unwrap();

        assert_eq!(pricing.buy_currency_rate, Some(dec!(1.0)));
        assert_eq!(pricing.sell_currency_rate, Some(dec!(16500.0)));

        // 49500 IDR = 3 USD → buy_price = 1000 → converted_buy = 1003
        assert!((pricing.converted_buy_price.unwrap() - dec!(1003.0)).abs() < dec!(0.01));
        assert!((pricing.sell_price - dec!(16549500.0)).abs() < dec!(0.01));

        // For markup_value_in_buy_currency (49500 IDR / 16500 IDR/USD = 3 USD)
        assert_eq!(pricing.markup_value_in_buy_currency, Some(dec!(3.0)));
    }

    #[test]
    fn test_markup_percentage() {
        let (usd, idr) = currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(dec!(1000.0), usd.clone(), idr.clone());
        pricing.markup = Some(MarkupType::Percentage(dec!(10.0)));
        pricing.apply_markup(&converter).unwrap();

        // 10% of 1000 = 100 → sell_base = 1100 → IDR: 1100 * 16500 = 18,150,000
        assert_eq!(pricing.markup_value_in_buy_currency.unwrap(), dec!(100.0)); 
        assert!((pricing.sell_price - dec!(18150000.0)).abs() < dec!(0.01)); 
    }

    #[test]
    fn test_markup_commission() {
        let (usd, idr) = currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(dec!(1000.0), usd.clone(), idr.clone());
        pricing.markup = Some(MarkupType::Commission(dec!(10.0)));
        pricing.apply_markup(&converter).unwrap();

        // 1000 / (1 - 0.1) = 1111.1111...
        // Check markup_value_in_buy_currency first:
        // markup = (buy_price * commission_pct) / (100 - commission_pct)
        // markup = (1000 * 10) / (100 - 10) = 10000 / 90 = 111.111...
        assert!((pricing.markup_value_in_buy_currency.unwrap() - dec!(111.111111)).abs() < dec!(0.000001));

        // converted_buy_price = buy_price + markup_value_in_buy_currency
        // 1000 + 111.111... = 1111.111...
        assert!((pricing.converted_buy_price.unwrap() - dec!(1111.111111)).abs() < dec!(0.000001));

        // sell_price = converted_buy_price * exchange_rate
        // 1111.111111 * 16500 = 18333333.3333...
        assert!((pricing.sell_price - dec!(18333333.333333)).abs() < dec!(0.01)); // Adjust delta for decimal precision
    }

    #[test]
    fn test_no_markup() {
        let (usd, idr) = currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(dec!(1000.0), usd.clone(), idr.clone());
        pricing.apply_markup(&converter).unwrap();

        assert_eq!(pricing.markup, None);
        assert_eq!(pricing.markup_value_in_buy_currency.unwrap(), dec!(0.0));
        assert_eq!(pricing.sell_price, dec!(16500000.0));
    }
}