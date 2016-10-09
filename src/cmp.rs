// https://github.com/rust-lang/rust/issues/13101

use ast;
use attr;
use quote;
use syn::{self, aster};
use utils;

/// Derive `Eq` for `input`.
pub fn derive_eq(input: &ast::Input, debug: &attr::InputEq) -> quote::Tokens {
    let name = &input.ident;

    let eq_trait_path = eq_trait_path();
    let impl_generics = utils::build_impl_generics(
        input,
        &eq_trait_path,
        |attrs| attrs.eq_bound().is_none(),
        |field| field.eq_bound(),
        |input| input.eq_bound(),
    );
    let where_clause = &impl_generics.where_clause;

    let ty = syn::aster::ty().path()
                             .segment(name.clone())
                             .with_generics(impl_generics.clone())
                             .build()
                             .build();

    quote!(
        impl #impl_generics #eq_trait_path for #ty #where_clause {}
    )
}

/// Return the path of the `Eq` trait, that is `::std::cmp::Eq`.
fn eq_trait_path() -> syn::Path {
    aster::path().global().ids(&["std", "cmp", "Eq"]).build()
}
