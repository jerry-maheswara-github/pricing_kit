#[cfg(test)]
mod tests {
    use pricing_kit::{AdjustmentKind, Currency, CurrencyConverter, MarkupType, PriceAdjustment, PricingDetail, dec};

    #[test]
    fn test_apply_tax_and_discount_adjustments() {
        let usd = Currency::new("USD", "US Dollar");
        let idr = Currency::new("IDR", "Indonesian Rupiah");

        let mut converter = CurrencyConverter::new();
        converter.add_exchange_rate(&usd, dec!(1.0));   
        converter.add_exchange_rate(&idr, dec!(16500.0));

        let mut pricing = PricingDetail::new(dec!(1000.0), usd.clone(), idr.clone());
        pricing.markup = Some(MarkupType::Percentage(dec!(10.0)));
        pricing.apply_markup(&converter).unwrap();

        let adjustments = vec![
            PriceAdjustment::Tax {
                name: "Tax 11%".into(),
                percentage: dec!(11.0),
            },
            PriceAdjustment::Discount {
                name: "Discount 5%".into(),
                percentage: dec!(5.0),
            },
        ];

        pricing.apply_adjustments(&adjustments, &converter).unwrap();

        // Manual calculation verification (important for decimal logic):
        // 1. Buy price in USD: 1000.0
        // 2. Markup 10%: 1000.0 * 0.10 = 100.0 USD
        // 3. Converted buy price (base for sell_price): 1000.0 + 100.0 = 1100.0 USD
        // 4. Sell price in IDR (before adjustments): 1100.0 * 16500.0 = 18,150,000.0 IDR

        // Now apply adjustments sequentially:
        // Current price: 18,150,000.0 IDR
        // 1. Tax 11%: 18,150,000.0 * 0.11 = 1,996,500.0 IDR
        //    Price after Tax: 18,150,000.0 + 1,996,500.0 = 20,146,500.0 IDR
        // 2. Discount 5%: 20,146,500.0 * 0.05 = 1,007,325.0 IDR
        //    Final Price: 20,146,500.0 - 1,007,325.0 = 19,139,175.0 IDR

        let expected_price = dec!(19139175.0);
        let tolerance = dec!(0.01);

        // Direct field access for pricing.sell_price
        assert!(
            (pricing.sell_price - expected_price).abs() < tolerance,
            "Expected final price {}, got {}",
            expected_price,
            pricing.sell_price
        );

        assert_eq!(pricing.applied_adjustments.len(), 2);

        let tax = &pricing.applied_adjustments[0];
        // Now comparing with AdjustmentKind enum
        assert_eq!(tax.kind, AdjustmentKind::Tax);
        assert_eq!(tax.name, "Tax 11%");
        assert!(tax.applied_amount > dec!(0.0));

        let discount = &pricing.applied_adjustments[1];
        // Now comparing with AdjustmentKind enum
        assert_eq!(discount.kind, AdjustmentKind::Discount);
        assert_eq!(discount.name, "Discount 5%");
        assert!(discount.applied_amount < dec!(0.0));
    }
}