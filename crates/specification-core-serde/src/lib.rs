#![forbid(unsafe_code)]
//! Versioned declarative rule documents for `specification-core` integrations.
//!
//! This crate serializes data that describes a rule graph. It does not and
//! cannot serialize arbitrary executable closures or trait objects.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// The current wire-schema version.
pub const CURRENT_SCHEMA_VERSION: u16 = 1;

/// A declarative rule graph that can be persisted or transported.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleNode {
    /// A rule that always succeeds.
    Always,
    /// A rule that never succeeds.
    Never,
    /// A rule requiring every child to succeed.
    AllOf {
        /// Child rules evaluated with logical AND.
        rules: Vec<Self>,
    },
    /// A rule requiring at least one child to succeed.
    AnyOf {
        /// Child rules evaluated with logical OR.
        rules: Vec<Self>,
    },
    /// A negated child rule.
    Not {
        /// Child rule to negate.
        rule: Box<Self>,
    },
    /// A strict counter limit.
    MaxCount {
        /// Counter name.
        key: String,
        /// Exclusive maximum count.
        maximum: u64,
    },
    /// A named boolean flag.
    Flag {
        /// Flag name.
        key: String,
    },
    /// A cooldown duration represented in milliseconds.
    Cooldown {
        /// Timestamp name.
        key: String,
        /// Required elapsed duration in milliseconds.
        duration_millis: u64,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum WireRuleNode {
    Always,
    Never,
    AllOf { rules: Vec<WireRuleNode> },
    AnyOf { rules: Vec<WireRuleNode> },
    Not { rule: Box<WireRuleNode> },
    MaxCount { key: String, maximum: u64 },
    Flag { key: String },
    Cooldown { key: String, duration_millis: u64 },
}

impl From<RuleNode> for WireRuleNode {
    fn from(node: RuleNode) -> Self {
        match node {
            RuleNode::Always => Self::Always,
            RuleNode::Never => Self::Never,
            RuleNode::AllOf { rules } => Self::AllOf {
                rules: rules.into_iter().map(Self::from).collect(),
            },
            RuleNode::AnyOf { rules } => Self::AnyOf {
                rules: rules.into_iter().map(Self::from).collect(),
            },
            RuleNode::Not { rule } => Self::Not {
                rule: Box::new((*rule).into()),
            },
            RuleNode::MaxCount { key, maximum } => Self::MaxCount { key, maximum },
            RuleNode::Flag { key } => Self::Flag { key },
            RuleNode::Cooldown {
                key,
                duration_millis,
            } => Self::Cooldown {
                key,
                duration_millis,
            },
        }
    }
}

impl From<WireRuleNode> for RuleNode {
    fn from(node: WireRuleNode) -> Self {
        match node {
            WireRuleNode::Always => Self::Always,
            WireRuleNode::Never => Self::Never,
            WireRuleNode::AllOf { rules } => Self::AllOf {
                rules: rules.into_iter().map(Self::from).collect(),
            },
            WireRuleNode::AnyOf { rules } => Self::AnyOf {
                rules: rules.into_iter().map(Self::from).collect(),
            },
            WireRuleNode::Not { rule } => Self::Not {
                rule: Box::new((*rule).into()),
            },
            WireRuleNode::MaxCount { key, maximum } => Self::MaxCount { key, maximum },
            WireRuleNode::Flag { key } => Self::Flag { key },
            WireRuleNode::Cooldown {
                key,
                duration_millis,
            } => Self::Cooldown {
                key,
                duration_millis,
            },
        }
    }
}

/// A versioned document containing one declarative rule graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDocument {
    rule: RuleNode,
}

impl RuleDocument {
    /// Creates a document using [`CURRENT_SCHEMA_VERSION`].
    pub const fn new(rule: RuleNode) -> Self {
        Self { rule }
    }

    /// Returns the document's declarative rule graph.
    pub const fn rule(&self) -> &RuleNode {
        &self.rule
    }

    /// Returns the schema version emitted by this crate.
    pub const fn schema_version(&self) -> u16 {
        CURRENT_SCHEMA_VERSION
    }
}

#[derive(Deserialize, Serialize)]
struct WireDocument {
    schema_version: u16,
    rule: WireRuleNode,
}

impl Serialize for RuleDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        WireDocument {
            schema_version: CURRENT_SCHEMA_VERSION,
            rule: self.rule.clone().into(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RuleDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let document = WireDocument::deserialize(deserializer)?;
        if document.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(de::Error::custom(UnsupportedSchemaVersion {
                received: document.schema_version,
            }));
        }
        Ok(Self::new(document.rule.into()))
    }
}

/// A schema version that this crate does not understand.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedSchemaVersion {
    received: u16,
}

impl UnsupportedSchemaVersion {
    /// Returns the version found on the wire.
    pub const fn received(&self) -> u16 {
        self.received
    }
}

impl std::fmt::Display for UnsupportedSchemaVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "unsupported rule schema version {}",
            self.received
        )
    }
}

impl std::error::Error for UnsupportedSchemaVersion {}

#[cfg(test)]
mod tests {
    use super::{CURRENT_SCHEMA_VERSION, RuleDocument, RuleNode};

    #[test]
    fn schema_round_trip_is_versioned_and_deterministic() {
        let document = RuleDocument::new(RuleNode::AllOf {
            rules: vec![
                RuleNode::Flag {
                    key: "premium".into(),
                },
                RuleNode::Not {
                    rule: Box::new(RuleNode::MaxCount {
                        key: "attempts".into(),
                        maximum: 3,
                    }),
                },
            ],
        });

        let json = serde_json::to_string(&document).expect("document should serialize");
        assert_eq!(
            json,
            r#"{"schema_version":1,"rule":{"kind":"all_of","rules":[{"kind":"flag","key":"premium"},{"kind":"not","rule":{"kind":"max_count","key":"attempts","maximum":3}}]}}"#
        );
        let decoded: RuleDocument = serde_json::from_str(&json).expect("document should decode");
        assert_eq!(decoded, document);
        assert_eq!(decoded.schema_version(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn unknown_schema_versions_are_rejected() {
        let error = serde_json::from_str::<RuleDocument>(
            r#"{"schema_version":99,"rule":{"kind":"always"}}"#,
        )
        .expect_err("unknown versions must not be silently accepted");

        assert!(
            error
                .to_string()
                .contains("unsupported rule schema version 99")
        );
    }
}
