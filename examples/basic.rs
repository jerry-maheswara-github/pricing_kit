use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    let mut pricing = PricingDetail::new(100.0, usd.clone(), idr.clone());
    pricing.set_markup(Some(MarkupType::Amount {
        value: 3500.0,
        currency: idr.clone(),
    }));

    pricing.apply_markup(&converter);

    println!("Pricing:\n{:#?}", pricing);
    
    // Manual calculation for comparative purpose:
    // 1. Buy price: 100.0 USD
    // 2. Markup: Amount 3500 IDR → converted to USD = 3500 / 16500 = ~0.2121 USD
    // 3. Converted buy price: 100 + 0.2121 = ~100.2121 USD
    // 4. Exchange rate USD → IDR = 16500
    // 5. Final sell price: 100.2121 * 16500 = 1_653_500.0 IDR


    Ok(())
}