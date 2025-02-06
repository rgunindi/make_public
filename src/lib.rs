use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn make_public(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;
    let fields = input.fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });
    let attrs = &input.attrs;

    let expanded = quote! {
        #(#attrs)*
        pub struct #struct_name {
            #(#fields,)*
        }
    };

    expanded.into()
}
