#[derive(Debug)]
pub struct PricingContext<'a> {
    pub vendor_id: Option<&'a str>,
    pub origin: Option<&'a str>,
    pub destination: Option<&'a str>,
    pub passenger_type: Option<&'a str>,
    pub currency_code: &'a str,
    pub base_amount: f64,
}
