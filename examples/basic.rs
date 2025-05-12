use pricing_kit::model::Currency;
use pricing_kit::model::{CurrencyConverter, Pricing};
fn main(){
    let usd = Currency::new("USD", "American Dollar");
    let idr = Currency::new("IDR", "Indonesian Rupiah");
    let mut pricing = Pricing::new(100.0, usd.clone());

    let mut converter = CurrencyConverter::new();
    converter.add_exchange_rate(&usd, 1.0);
    converter.add_exchange_rate(&idr, 16500.0);

    pricing.set_sell_price_by_amount(50.0, &converter);
    let sell_price_in_idr = pricing.convert_sell_price(&converter, &idr);

    println!("converter: {:?}", converter);
    println!("pricing: {:?}", pricing);
    println!("Sell price in IDR: {}", sell_price_in_idr.abs());
}