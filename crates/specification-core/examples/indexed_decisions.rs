//! Routes candidates through an indexed decision catalog.

use specification_core::{DecisionSpecification, IndexedFirstMatch};

struct Artifact<'a> {
    name: &'a str,
    is_directory: bool,
}

fn main() {
    let mut classifier = IndexedFirstMatch::new(|item: &Artifact<'_>| item.name);
    classifier.push_keyed(
        "target",
        |item: &Artifact<'_>| item.name == "target" && item.is_directory,
        "rust-build",
    );
    classifier.push_unkeyed(
        |item: &Artifact<'_>| item.name.ends_with(".egg-info"),
        "python-metadata",
    );

    let build = Artifact {
        name: "target",
        is_directory: true,
    };
    assert_eq!(classifier.decide(&build), Some(&"rust-build"));
    assert!(classifier.may_match("target"));
}
