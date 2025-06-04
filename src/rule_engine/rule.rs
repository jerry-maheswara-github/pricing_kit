use serde::{Deserialize, Serialize};
use crate::{MarkupType, PriceAdjustment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RuleCondition {
    Vendor { id: String },
    RouteOrigin { origin: String },
    RouteDestination { destination: String },
    Currency { code: String },
    PassengerType { r#type: String },
    And { conditions: Vec<RuleCondition> },
    Or { conditions: Vec<RuleCondition> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Rule {
    /// Apply a markup if this rule matches.
    Markup {
        markup: MarkupType,
    },

    /// Apply an adjustment if this rule matches.
    Adjustment {
        adjustment: PriceAdjustment,
    },

    /// Rule with condition wrapping another rule.
    Conditioned {
        condition: RuleCondition,
        rule: Box<Rule>,
    },
}
