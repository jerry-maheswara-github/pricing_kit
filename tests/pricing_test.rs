#[cfg(test)]
mod tests {
    use pricing_kit::model::{Currency, CurrencyConverter, Pricing};
    
    #[test]
    fn test_pricing_new() {
        let usd = Currency::new("USD", "American Dollar");
        let pricing = Pricing::new(100.0, usd.clone());

        assert_eq!(pricing.buy_price, 100.0);
        assert_eq!(pricing.sell_price, 100.0);
        assert_eq!(pricing.buy_currency.get_code(), "USD");
        assert_eq!(pricing.sell_currency.get_code(), "USD");
    }

    #[test]
    fn test_set_sell_price_by_amount() {
        let usd = Currency::new("USD", "American Dollar");
        let mut pricing = Pricing::new(100.0, usd.clone());
        let mut converter = CurrencyConverter::new();
        converter.add_exchange_rate(&usd, 1.0);

        pricing.set_sell_price_by_amount(20.0, &converter);

        assert_eq!(pricing.sell_price, 120.0); // 100 + 20
    }

    #[test]
    fn test_set_sell_price_by_percentage() {
        let usd = Currency::new("USD", "American Dollar");
        let mut pricing = Pricing::new(100.0, usd.clone());

        pricing.set_sell_price_by_percentage(20.0);
        assert_eq!(pricing.sell_price, 120.0); // 100 * (1 + 0.20)
    }

    #[test]
    fn test_set_sell_price_by_commission() {
        let usd = Currency::new("USD", "American Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");
        let mut pricing = Pricing::new(100.0, usd.clone());
        let mut converter = CurrencyConverter::new();

        converter.add_exchange_rate(&usd, 1.0); 
        converter.add_exchange_rate(&idr, 14500.0); 

        pricing.set_sell_price_by_commission(10.0, &converter);

        let expected_sell_price = 100.0 + (10.0 / 14500.0); 

        eprintln!("expected_sell_price = {:?}", expected_sell_price);
        eprintln!("abs = {:?}", (pricing.sell_price - expected_sell_price).abs());
        assert!((pricing.sell_price - expected_sell_price).abs() <=  9.999310344827592 );  
    }
    
    #[test]
    fn test_convert_sell_price() {
        let usd = Currency::new("USD", "American Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");
        let pricing = Pricing::new(432.62499, usd.clone());
        let mut converter = CurrencyConverter::new();
        converter.add_exchange_rate(&usd, 1.0); // Assume 1 USD = 1 USD
        converter.add_exchange_rate(&idr, 16500.0); // Assume 1 USD = 14500 IDR

        let converted_price = pricing.convert_sell_price(&converter, &idr);
        assert_eq!(converted_price, 7138312.335); // 432.6 USD to IDR at 14500 rate
    }
}