use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType, dec, PriceAdjustment, ToPrimitive};

fn main(){
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, dec!(1.0));
    converter.add_exchange_rate(&idr, dec!(16500.0));

    let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());
    pricing.markup = Some(MarkupType::Percentage(dec!(20.0)));
    pricing.apply_markup(&converter).expect("Failed to apply markup");

    let adjustments = vec![
        PriceAdjustment::Tax {
            name: "Tax 11%".into(),
            percentage: dec!(11.0),
        },
        PriceAdjustment::Discount {
            name: "Discount".into(),
            percentage: dec!(5.0),
        },
        PriceAdjustment::Fixed {
            name: "Promo New Year".to_string(),
            amount: dec!(10.0),
            currency: pricing.sell_currency.clone(),
        }
    ];

    pricing.apply_adjustments(&adjustments, &converter).expect("Failed to apply adjustments");
    
    println!("Adjustment Pricing:\n{:#?}", pricing);

    // Manual calculation for comparative purpose:
    // 1. Buy price in USD: 100.0
    // 2. Markup 20% -> 100 * 0.2 = 20 USD
    // 3. Converted buy price: 100 + 20 = 120 USD
    // 4. Sell price in IDR: 120 * 16500 = 1_980_000 IDR

    // Adjustments:
    // + Tax 11% of 1_980_000 = 217_800 -> 2_197_800
    // - Discount 5% of 2_197_800 = 109_890 -> 2_087_910
    // + Fixed amount 10 IDR -> final = 2_087_920.0 IDR
    
    if let Some(total_f64) = pricing.sell_price.to_f64() {
        println!("Total sell price as f64: {}", total_f64);
    }

    // Ok(())
}