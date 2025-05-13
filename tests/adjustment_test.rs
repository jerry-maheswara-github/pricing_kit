#[cfg(test)]
mod tests {
    use pricing_kit::{Currency, CurrencyConverter, MarkupType, PriceAdjustment, PricingDetail};

    #[test]
    fn test_apply_tax_and_discount_adjustments() {
        let usd = Currency::new("USD", "US Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");

        let mut converter = CurrencyConverter::new();
        converter.add_exchange_rate(&usd, 1.0);
        converter.add_exchange_rate(&idr, 16500.0);

        let mut pricing = PricingDetail::new(1000.0, usd.clone(), idr.clone());
        pricing.set_markup(MarkupType::Percentage(10.0));
        pricing.apply_markup(&converter);

        let adjustments = vec![
            PriceAdjustment::Tax {
                name: "Tax 11%".into(),
                percentage: 11.0,
            },
            PriceAdjustment::Discount {
                name: "Discount 5%".into(),
                percentage: 5.0,
            },
        ];

        pricing.apply_adjustments(&adjustments, &converter);

        // Sell price after markup = 1000 + 10% = 1100 USD
        // Convert to IDR: 1100 * 16500 = 18,150,000
        // Tax 11% = 1,996,500
        // Discount 5% from (18150000 + 1996500) = 20116500 * 5% = 1,007,325
        // Final price = 20116500 - 1007325 = 19,139,175

        let expected_price = 19139175.0;
        let tolerance = 0.01;

        assert!(
            (pricing.get_sell_price() - expected_price).abs() < tolerance,
            "Expected final price {}, got {}",
            expected_price,
            pricing.get_sell_price()
        );

        assert_eq!(pricing.applied_adjustments.len(), 2);

        let tax = &pricing.applied_adjustments[0];
        assert_eq!(tax.kind, "Tax");
        assert_eq!(tax.name, "Tax 11%");
        assert!(tax.applied_amount > 0.0);

        let discount = &pricing.applied_adjustments[1];
        assert_eq!(discount.kind, "Discount");
        assert_eq!(discount.name, "Discount 5%");
        assert!(discount.applied_amount < 0.0);
    }
}
