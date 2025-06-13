#[cfg(test)]
mod tests {
    use pricing_kit::{Currency, CurrencyConverter, MarkupType, PricingDetail};

    fn setup_converter() -> CurrencyConverter {
        let mut converter = CurrencyConverter::new();
        let usd = Currency::new("USD", "US Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");

        converter.add_exchange_rate(&usd, 1.0);        // USD as base
        converter.add_exchange_rate(&idr, 16500.0);    // 1 USD = 16,500 IDR
        converter
    }

    fn get_currencies() -> (Currency, Currency) {
        (
            Currency::new("USD", "US Dollar"),
            Currency::new("IDR", "Indonesian Rupiah"),
        )
    }

    #[test]
    fn test_markup_amount_in_idr() {
        let (usd, idr) = get_currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(1000.0, usd.clone(), idr.clone());
        pricing.set_markup(Some(MarkupType::Amount {
            value: 49500.0,
            currency: idr.clone(),
        }));

        pricing.apply_markup(&converter);

        assert_eq!(pricing.get_buy_currency_rate(), Some(1.0));
        assert_eq!(pricing.get_sell_currency_rate(), Some(16500.0));

        // 49500 IDR = 3 USD → buy_price = 1000 → converted_buy = 1003
        assert!((pricing.get_converted_buy_price().unwrap() - 1003.0).abs() < 0.01);
        assert!((pricing.get_sell_price() - 16549500.0).abs() < 0.01);
    }

    #[test]
    fn test_markup_percentage() {
        let (usd, idr) = get_currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(1000.0, usd.clone(), idr.clone());
        pricing.set_markup(Some(MarkupType::Percentage(10.0)));
        pricing.apply_markup(&converter);

        // 10% of 1000 = 100 → sell_base = 1100 → IDR: 1100 * 16500 = 18,150,000
        assert_eq!(pricing.get_markup_value_in_buy_currency().unwrap(), 100.0);
        assert!((pricing.get_sell_price() - 18150000.0).abs() < 0.01);
    }

    #[test]
    fn test_markup_commission() {
        let (usd, idr) = get_currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(1000.0, usd.clone(), idr.clone());
        pricing.set_markup(Some(MarkupType::Commission(10.0)));
        pricing.apply_markup(&converter);

        // 1000 / (1 - 0.1) = 1111.11
        assert!((pricing.get_converted_buy_price().unwrap() - 1111.11).abs() < 0.1);
    }

    #[test]
    fn test_no_markup() {
        let (usd, idr) = get_currencies();
        let converter = setup_converter();

        let mut pricing = PricingDetail::new(1000.0, usd.clone(), idr.clone());
        pricing.apply_markup(&converter);

        assert_eq!(pricing.get_markup(), &None);
        assert_eq!(pricing.get_markup_value_in_buy_currency().unwrap(), 0.0);
        assert_eq!(pricing.get_sell_price(), 16500000.0);
    }
}
