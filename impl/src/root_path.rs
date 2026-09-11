pub(crate) fn root_path() -> proc_macro2::TokenStream {
    // Using `::hicore::thiserror` here instead of simply `thiserror` makes the macros unusable inside
    // `hicore` itself, but un-ambiguous everywhere else.
    quote::quote! {
        ::hicore::thiserror
    }
}
