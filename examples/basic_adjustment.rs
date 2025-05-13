use pricing_kit::{Currency, CurrencyConverter, PricingDetail, MarkupType};
use pricing_kit::PriceAdjustment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    let mut pricing = PricingDetail::new(100.0, usd.clone(), idr.clone());
    pricing.set_markup(MarkupType::Percentage(20.0));
    pricing.apply_markup(&converter);

    let adjustments = vec![
        PriceAdjustment::Tax {
            name: "Tax 11%".into(),
            percentage: 11.0,
        },
        PriceAdjustment::Discount {
            name: "Discount".into(),
            percentage: 5.0,
        },
        PriceAdjustment::Fixed {
            name: "Promo New Year".to_string(),
            amount: 10.0,
            currency: pricing.sell_currency.clone(),
        }
    ];

    pricing.apply_adjustments(&adjustments, &converter);

    let json = serde_json::to_string_pretty(&pricing)?;
    println!("==================\nAdjustment Pricing:\n{}", json);

    Ok(())
}