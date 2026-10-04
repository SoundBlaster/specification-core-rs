#![forbid(unsafe_code)]
//! Opt-in procedural macros for `specification-core`.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, FnArg, Ident, ItemFn, LitStr, ReturnType, Token, Type, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

/// Turns a predicate function into a named `specification_core::Specification`
/// unit struct.
///
/// The function must have the shape `fn(&Candidate) -> bool`. The attribute
/// argument is the generated specification type name:
///
/// The snippet is illustrative; the workspace integration test compiles the
/// expansion against the core crate.
///
/// ```ignore
/// #[specification_core_macros::specification(AdultSpec)]
/// fn is_adult(candidate: &User) -> bool { candidate.age >= 18 }
/// ```
#[proc_macro_attribute]
pub fn specification(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let specification_name = parse_macro_input!(attribute as Ident);
    let function = parse_macro_input!(item as ItemFn);

    match expand_specification(specification_name, function) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generates a concrete, ordered keyed decision evaluator.
///
/// `context`, `decision`, and `key` declare the evaluator types and borrowed
/// string projection. Rules use `keyed(["key", ...], specification, decision)`
/// or `unkeyed(specification, decision)`. Every specification and decision
/// expression is evaluated once when the macro expression is constructed, and
/// `decide` returns a borrow of the stored decision.
/// Evaluation projects the key once and checks only the keyed rules for that
/// key plus every unkeyed rule, in declaration order. A keyed specification
/// must never match a candidate whose projected key is different from one of
/// its declared aliases.
///
/// This snippet uses the macro as a downstream crate would; the integration
/// test compiles it against both workspace crates, while rustdoc cannot add the
/// macro crate as an example consumer dependency here.
///
/// ```ignore
/// use specification_core::DecisionSpecification;
/// use specification_core_macros::first_match;
///
/// struct Facts { name: String, directory: bool }
/// let evaluator = first_match! {
///     context: Facts,
///     decision: u16,
///     key: |facts: &Facts| facts.name.as_str(),
///     rules: [
///         keyed(["target"], |facts: &Facts| facts.name == "target" && facts.directory, 1)
///     ]
/// };
/// assert_eq!(evaluator.decide(&Facts { name: "target".into(), directory: true }), Some(&1));
/// ```
#[proc_macro]
pub fn first_match(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FirstMatchInput);
    match expand_first_match(input) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct FirstMatchInput {
    context: Type,
    decision: Type,
    key: Expr,
    rules: Vec<DecisionRule>,
}

enum DecisionRule {
    Keyed {
        keys: Vec<LitStr>,
        specification: Expr,
        decision: Expr,
    },
    Unkeyed {
        specification: Expr,
        decision: Expr,
    },
}

impl Parse for FirstMatchInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        parse_named_field(input, "context")?;
        let context = input.parse()?;
        input.parse::<Token![,]>()?;
        parse_named_field(input, "decision")?;
        let decision = input.parse()?;
        input.parse::<Token![,]>()?;
        parse_named_field(input, "key")?;
        let key = input.parse()?;
        input.parse::<Token![,]>()?;
        parse_named_field(input, "rules")?;

        let content;
        bracketed!(content in input);
        let rules = Punctuated::<DecisionRule, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        if !input.is_empty() {
            return Err(input.error("unexpected tokens after `rules`"));
        }

        Ok(Self {
            context,
            decision,
            key,
            rules,
        })
    }
}

fn parse_named_field(input: ParseStream<'_>, expected: &str) -> syn::Result<()> {
    let name: Ident = input.parse()?;
    if name != expected {
        return Err(syn::Error::new(
            name.span(),
            format!("expected `{expected}:` in first_match! input"),
        ));
    }
    input.parse::<Token![:]>()?;
    Ok(())
}

impl Parse for DecisionRule {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let kind: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        match kind.to_string().as_str() {
            "keyed" => {
                let key_content;
                bracketed!(key_content in content);
                let keys = Punctuated::<LitStr, Token![,]>::parse_terminated(&key_content)?
                    .into_iter()
                    .collect::<Vec<_>>();
                if keys.is_empty() {
                    return Err(syn::Error::new(
                        key_content.span(),
                        "a `keyed` rule needs at least one string key",
                    ));
                }
                content.parse::<Token![,]>()?;
                let specification = content.parse()?;
                content.parse::<Token![,]>()?;
                let decision = content.parse()?;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
                ensure_empty(&content)?;
                Ok(Self::Keyed {
                    keys,
                    specification,
                    decision,
                })
            }
            "unkeyed" => {
                let specification = content.parse()?;
                content.parse::<Token![,]>()?;
                let decision = content.parse()?;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
                ensure_empty(&content)?;
                Ok(Self::Unkeyed {
                    specification,
                    decision,
                })
            }
            _ => Err(syn::Error::new(
                kind.span(),
                "expected `keyed([\"key\"], specification, decision)` or `unkeyed(specification, decision)`",
            )),
        }
    }
}

fn ensure_empty(input: ParseStream<'_>) -> syn::Result<()> {
    if input.is_empty() {
        Ok(())
    } else {
        Err(input.error("unexpected tokens in decision rule"))
    }
}

fn expand_first_match(input: FirstMatchInput) -> syn::Result<proc_macro2::TokenStream> {
    let FirstMatchInput {
        context,
        decision,
        key,
        rules,
    } = input;
    let evaluator_name = format_ident!("__StaticFirstMatch");
    let projection_type = format_ident!("__KeyProjection");
    let context_type = format_ident!("__Context");
    let decision_type = format_ident!("__Decision");
    let specification_types = (0..rules.len())
        .map(|index| format_ident!("__Specification{index}"))
        .collect::<Vec<_>>();
    let specification_fields = (0..rules.len())
        .map(|index| format_ident!("specification_{index}"))
        .collect::<Vec<_>>();
    let decision_fields = (0..rules.len())
        .map(|index| format_ident!("decision_{index}"))
        .collect::<Vec<_>>();
    // Local items cannot capture generic parameters or lifetimes from the
    // surrounding function. Infer independent generated type parameters from
    // the initializer instead.
    let mut generic_types = vec![
        context_type.clone(),
        decision_type.clone(),
        projection_type.clone(),
    ];
    generic_types.extend(specification_types.iter().cloned());

    let mut keys = Vec::<LitStr>::new();
    for rule in &rules {
        if let DecisionRule::Keyed { keys: aliases, .. } = rule {
            for alias in aliases {
                if !keys
                    .iter()
                    .any(|existing| existing.value() == alias.value())
                {
                    keys.push(alias.clone());
                }
            }
        }
    }

    let unkeyed_indices = rules
        .iter()
        .enumerate()
        .filter_map(|(index, rule)| matches!(rule, DecisionRule::Unkeyed { .. }).then_some(index))
        .collect::<Vec<_>>();
    let emit_rule = |index: usize| {
        let specification_field = &specification_fields[index];
        let decision_field = &decision_fields[index];
        quote! {
            if ::specification_core::Specification::is_satisfied_by(
                &self.#specification_field,
                candidate,
            ) {
                return ::core::option::Option::Some(&self.#decision_field);
            }
        }
    };
    let emit_arm = |alias: &LitStr| {
        let matching_indices = rules
            .iter()
            .enumerate()
            .filter_map(|(index, rule)| match rule {
                DecisionRule::Keyed { keys: aliases, .. }
                    if aliases.iter().any(|key| key.value() == alias.value()) =>
                {
                    Some(index)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let ordered = (0..rules.len())
            .filter(|index| matching_indices.contains(index) || unkeyed_indices.contains(index))
            .collect::<Vec<_>>();
        let evaluations = ordered.into_iter().map(emit_rule);
        quote! {
            #alias => {
                #(#evaluations)*
            }
        }
    };
    let arms = keys.iter().map(emit_arm);
    let default_evaluations = unkeyed_indices.iter().copied().map(emit_rule);
    let specification_bounds = specification_types.iter().map(|specification_type| {
        quote! { #specification_type: ::specification_core::Specification<#context_type> }
    });
    let projection_bound = quote! {
        #projection_type: for<'candidate> ::core::ops::Fn(
            &'candidate #context_type
        ) -> &'candidate str
    };
    let rule_fields = (0..rules.len()).map(|index| {
        let specification_field = &specification_fields[index];
        let specification_type = &specification_types[index];
        let decision_field = &decision_fields[index];
        quote! {
            #specification_field: #specification_type,
            #decision_field: #decision_type,
        }
    });
    let rule_initializers = rules.iter().enumerate().map(|(index, rule)| {
        let specification_field = &specification_fields[index];
        let decision_field = &decision_fields[index];
        let specification = match rule {
            DecisionRule::Keyed { specification, .. }
            | DecisionRule::Unkeyed { specification, .. } => specification,
        };
        let value = match rule {
            DecisionRule::Keyed { decision, .. } | DecisionRule::Unkeyed { decision, .. } => {
                decision
            }
        };
        quote! {
            #specification_field: (#specification),
            #decision_field: (#value),
        }
    });

    Ok(quote! {{
        fn __require_key_projection<Projection, Context>(projection: Projection) -> Projection
        where
            Projection: for<'candidate> ::core::ops::Fn(
                &'candidate Context
            ) -> &'candidate str,
        {
            projection
        }

        struct #evaluator_name<#(#generic_types),*> {
            _context: ::core::marker::PhantomData<(fn(&#context_type), fn() -> #decision_type)>,
            key: #projection_type,
            #(#rule_fields)*
        }

        impl<#(#generic_types),*> ::specification_core::DecisionSpecification<#context_type>
            for #evaluator_name<#(#generic_types),*>
        where
            #projection_bound,
            #(#specification_bounds,)*
        {
            type Decision = #decision_type;

            fn decide(&self, candidate: &#context_type) -> ::core::option::Option<&Self::Decision> {
                let key = (self.key)(candidate);
                match key {
                    #(#arms,)*
                    _ => {
                        #(#default_evaluations)*
                    }
                }
                ::core::option::Option::None
            }
        }

        #evaluator_name {
            _context: ::core::marker::PhantomData::<(
                fn(&#context), fn() -> #decision
            )>,
            key: __require_key_projection::<_, #context>(#key),
            #(#rule_initializers)*
        }
    }})
}

fn expand_specification(
    specification_name: Ident,
    function: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    if function.sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            function.sig.inputs.span(),
            "a specification predicate must accept exactly one shared reference argument",
        ));
    }

    let candidate_type = match function.sig.inputs.first().expect("length checked") {
        FnArg::Typed(argument) => match argument.ty.as_ref() {
            Type::Reference(reference) if reference.mutability.is_none() => {
                reference.elem.as_ref().clone()
            }
            _ => {
                return Err(syn::Error::new(
                    argument.ty.span(),
                    "the predicate argument must be an immutable reference, such as &User",
                ));
            }
        },
        FnArg::Receiver(receiver) => {
            return Err(syn::Error::new(
                receiver.span(),
                "a specification predicate must be a free function, not a method",
            ));
        }
    };

    match &function.sig.output {
        ReturnType::Type(_, output) if matches!(output.as_ref(), Type::Path(path) if path.path.is_ident("bool")) =>
            {}
        output => {
            return Err(syn::Error::new(
                output.span(),
                "a specification predicate must return bool",
            ));
        }
    }

    let function_name = &function.sig.ident;
    let visibility = &function.vis;
    Ok(quote! {
        #function

        #visibility struct #specification_name;

        impl ::specification_core::Specification<#candidate_type> for #specification_name {
            fn is_satisfied_by(&self, candidate: &#candidate_type) -> bool {
                #function_name(candidate)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use syn::{Ident, ItemFn, parse_quote};

    use super::{FirstMatchInput, expand_first_match, expand_specification};

    #[test]
    fn expansion_contains_the_named_specification_impl() {
        let function: ItemFn = parse_quote! {
            pub fn is_adult(candidate: &User) -> bool { candidate.age >= 18 }
        };
        let expanded = expand_specification(
            Ident::new("AdultSpec", proc_macro2::Span::call_site()),
            function,
        )
        .expect("valid predicate should expand")
        .to_string();

        assert!(expanded.contains("AdultSpec"));
        assert!(expanded.contains("Specification < User >"));
    }

    #[test]
    fn invalid_predicate_shape_returns_a_diagnostic() {
        let function: ItemFn = parse_quote! {
            fn invalid(candidate: User) -> String { candidate.name }
        };
        let error = expand_specification(
            Ident::new("InvalidSpec", proc_macro2::Span::call_site()),
            function,
        )
        .expect_err("invalid predicate must be rejected");

        assert!(error.to_string().contains("immutable reference"));
    }

    #[test]
    fn first_match_expands_to_concrete_fields_and_borrowed_dispatch() {
        let input = syn::parse_str::<FirstMatchInput>(
            r#"context: Facts, decision: u16, key: |facts: &Facts| facts.name.as_str(), rules: [keyed(["a", "alias"], IsA, 1), unkeyed(Fallback, 2)]"#,
        )
        .expect("valid first_match DSL should parse");
        let expanded = expand_first_match(input)
            .expect("valid rules should expand")
            .to_string();

        assert!(expanded.contains("DecisionSpecification < __Context >"));
        assert!(expanded.contains("__require_key_projection :: < _ , Facts >"));
        assert!(expanded.contains("match key"));
        assert!(!expanded.contains("Box <"));
        assert!(expanded.contains("\"a\" =>"));
        assert!(expanded.contains("\"alias\" =>"));
    }

    #[test]
    fn malformed_rules_return_actionable_diagnostics() {
        let empty_aliases = syn::parse_str::<FirstMatchInput>(
            r#"context: Facts, decision: u16, key: key_of, rules: [keyed([], IsA, 1)]"#,
        )
        .err()
        .expect("empty aliases must be rejected");
        assert!(
            empty_aliases
                .to_string()
                .contains("at least one string key")
        );

        let unknown_rule = syn::parse_str::<FirstMatchInput>(
            r#"context: Facts, decision: u16, key: key_of, rules: [dynamic(IsA, 1)]"#,
        )
        .err()
        .expect("unknown rule kinds must be rejected");
        assert!(unknown_rule.to_string().contains("expected `keyed"));
    }
}
