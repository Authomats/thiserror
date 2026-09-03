pub(crate) fn root_path() -> proc_macro2::TokenStream {
    // Simply use `thiserror` instead of `thiserror`. `::hicore::thiserror` would be more stable,
    // but make problems inside `hicore` itself.
    quote::quote! {
        thiserror
    }
}
