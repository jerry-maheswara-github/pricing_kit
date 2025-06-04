use super::rule::*;
use std::num::ParseFloatError;
use crate::{MarkupType, PriceAdjustment};

#[derive(Debug)]
pub enum DslError {
    InvalidSyntax(String),
    UnknownCondition(String),
    UnknownAction(String),
    ParseFloatError(ParseFloatError),
}

impl From<ParseFloatError> for DslError {
    fn from(e: ParseFloatError) -> Self {
        DslError::ParseFloatError(e)
    }
}

pub fn parse_dsl_rule(line: &str) -> Result<Rule, DslError> {
    let line = line.trim();

    if !line.starts_with("IF ") {
        return Err(DslError::InvalidSyntax("Missing IF".into()));
    }

    let parts: Vec<&str> = line[3..].splitn(2, " THEN ").collect(); // splitn(2) supaya aman
    if parts.len() != 2 {
        return Err(DslError::InvalidSyntax("Missing THEN or invalid format".into()));
    }

    let condition_str = parts[0].trim();
    let action_str = parts[1].trim();

    let condition = parse_condition(condition_str)?;
    let action_rule = parse_action(action_str)?;

    Ok(Rule::Conditioned {
        condition,
        rule: Box::new(action_rule),
    })
}

pub fn parse_condition(cond_str: &str) -> Result<RuleCondition, DslError> {
    // Contoh implementasi sederhana: parse AND (bisa dikembangkan ke OR, NOT nanti)
    let parts: Vec<&str> = cond_str.split(" AND ").collect();

    let mut conditions = Vec::with_capacity(parts.len());

    for part in parts {
        let part = part.trim();

        let cond = if let Some(val) = part.strip_prefix("vendor == ") {
            RuleCondition::Vendor { id: trim_quotes(val) }
        } else if let Some(val) = part.strip_prefix("origin == ") {
            RuleCondition::RouteOrigin { origin: trim_quotes(val) }
        } else if let Some(val) = part.strip_prefix("destination == ") {
            RuleCondition::RouteDestination { destination: trim_quotes(val) }
        } else if let Some(val) = part.strip_prefix("currency == ") {
            RuleCondition::Currency { code: trim_quotes(val) }
        } else {
            return Err(DslError::UnknownCondition(part.to_string()));
        };

        conditions.push(cond);
    }

    if conditions.len() == 1 {
        Ok(conditions.into_iter().next().unwrap())
    } else {
        Ok(RuleCondition::And { conditions })
    }
}

fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').to_string()
}


fn parse_action(action_str: &str) -> Result<Rule, DslError> {
    let action_str = action_str.trim();

    if let Some(pct_str) = action_str.strip_prefix("markup ") {
        let val = parse_percentage(pct_str)?;
        Ok(Rule::Markup {
            markup: MarkupType::Percentage(val),
        })
    } else if let Some(pct_str) = action_str.strip_prefix("tax ") {
        let val = parse_percentage(pct_str)?;
        Ok(Rule::Adjustment {
            adjustment: PriceAdjustment::Tax {
                name: "Tax".into(),
                percentage: val,
            },
        })
    } else {
        Err(DslError::UnknownAction(action_str.to_string()))
    }
}

fn parse_percentage(s: &str) -> Result<f64, DslError> {
    let s = s.trim_end_matches('%').trim();
    let val: f64 = s.parse()?;
    Ok(val)
}
