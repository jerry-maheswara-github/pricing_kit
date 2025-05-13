use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    let mut pricing = PricingDetail::new(100.0, usd.clone(), idr.clone());
    pricing.set_markup(MarkupType::Amount {
        value: 3500.0,
        currency: idr.clone(),
    });

    pricing.apply_markup(&converter);

    let json = serde_json::to_string_pretty(&pricing)?;
    println!("Pricing:\n{}", json);

    Ok(())
}