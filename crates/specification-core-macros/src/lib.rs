#![forbid(unsafe_code)]
//! Opt-in procedural macros for `specification-core`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, ReturnType, Type, parse_macro_input, spanned::Spanned};

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

    use super::expand_specification;

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
}
