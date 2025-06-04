use pricing_kit::{MarkupType, PriceAdjustment};
use pricing_kit::rule_engine::dsl::parse_dsl_rule;
use pricing_kit::rule_engine::rule::{Rule, RuleCondition};

#[test]
fn test_parse_markup_rule() {
    let dsl = r#"IF vendor == "GA" AND origin == "CGK" THEN markup 10%"#;
    let rule = parse_dsl_rule(dsl).expect("Failed to parse DSL");

    match rule {
        Rule::Conditioned { condition, rule } => {
            match *rule {
                Rule::Markup { markup: MarkupType::Percentage(pct) } => {
                    assert_eq!(pct, 10.0);
                }
                _ => panic!("Expected markup rule"),
            }

            match condition {
                RuleCondition::And { conditions } => {
                    assert_eq!(conditions.len(), 2);
                    match &conditions[0] {
                        RuleCondition::Vendor { id } => assert_eq!(id, "GA"),
                        _ => panic!("Expected vendor condition"),
                    }
                    match &conditions[1] {
                        RuleCondition::RouteOrigin { origin } => assert_eq!(origin, "CGK"),
                        _ => panic!("Expected origin condition"),
                    }
                }
                _ => panic!("Expected AND condition"),
            }
        }
        _ => panic!("Expected conditioned rule"),
    }
}

#[test]
fn test_parse_tax_rule() {
    let dsl = r#"IF currency == "IDR" THEN tax 11%"#;
    let rule = parse_dsl_rule(dsl).expect("Failed to parse DSL");

    match rule {
        Rule::Conditioned { condition, rule } => {
            match *rule {
                Rule::Adjustment {
                    adjustment: PriceAdjustment::Tax { name, percentage },
                } => {
                    assert_eq!(name, "Tax");
                    assert_eq!(percentage, 11.0);
                }
                _ => panic!("Expected tax adjustment"),
            }

            match condition {
                RuleCondition::Currency { code } => {
                    assert_eq!(code, "IDR");
                }
                _ => panic!("Expected currency condition"),
            }
        }
        _ => panic!("Expected conditioned rule"),
    }
}

#[test]
fn test_invalid_dsl() {
    let dsl = "currency == \"IDR\" THEN tax 11%"; // Missing IF
    assert!(parse_dsl_rule(dsl).is_err());
}
