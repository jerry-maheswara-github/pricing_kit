use super::{rule::*, context::PricingContext};

/// Fungsi ini belum memodifikasi pricing_detail langsung.
/// Tujuannya: kembalikan semua rule yang match, lalu kamu bisa apply manual atau otomatis.
pub fn filter_matching_rules<'a>(
    rules: &'a [Rule],
    context: &PricingContext,
) -> Vec<&'a Rule> {
    rules
        .iter()
        .filter(|rule| match rule {
            Rule::Conditioned { condition, .. } => evaluate_condition(condition, context),
            _ => true,
        })
        .collect()
}

pub fn evaluate_condition(cond: &RuleCondition, ctx: &PricingContext) -> bool {
    match cond {
        RuleCondition::Vendor { id } => ctx.vendor_id == Some(id.as_str()),
        RuleCondition::RouteOrigin { origin } => ctx.origin == Some(origin.as_str()),
        RuleCondition::RouteDestination { destination } => ctx.destination == Some(destination.as_str()),
        RuleCondition::Currency { code } => ctx.currency_code.eq_ignore_ascii_case(code),
        RuleCondition::PassengerType { r#type } => ctx.passenger_type == Some(r#type.as_str()),
        RuleCondition::And { conditions } => conditions.iter().all(|c| evaluate_condition(c, ctx)),
        RuleCondition::Or { conditions } => conditions.iter().any(|c| evaluate_condition(c, ctx)),
    }
}
