//! Serializes and deserializes a versioned declarative rule document.

use specification_core_serde::{RuleDocument, RuleNode};

fn main() {
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
    println!("{json}");

    let decoded: RuleDocument = serde_json::from_str(&json).expect("document should deserialize");
    assert_eq!(decoded, document);
}
