use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType, dec, ToPrimitive};

fn main() {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, dec!(1.0));
    converter.add_exchange_rate(&idr, dec!(16500.0));

    let mut pricing = PricingDetail::new(dec!(100.0), usd.clone(), idr.clone());

    // Access the `markup` field directly
    pricing.markup = Some(MarkupType::Amount {
        value: dec!(3500),
        currency: idr.clone(),
    });

    // Call `apply_markup` or `calculate_final_price`
    pricing.apply_markup(&converter).expect("Failed to apply markup");

    println!("Pricing after markup:\n{:#?}", pricing);

    // Manual calculation for comparative purpose:
    // 1. Buy price: 100.0 USD
    // 2. Markup: Amount 3500 IDR → converted to USD = 3500 / 16500 = ~0.21212121 USD
    // 3. Converted buy price: 100 + 0.21212121 = ~100.21212121 USD
    // 4. Exchange rate USD → IDR = 16500
    // 5. Final sell price: 100.21212121 * 16500 = 1_653_500.0 IDR

    if let Some(total_f64) = pricing.sell_price.to_f64() {
        println!("Total sell price as f64: {}", total_f64);
    }

}