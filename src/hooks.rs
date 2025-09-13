use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Ident, LitStr};

#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_item.sig.ident;
    let hook_name = parse_macro_input!(attr as LitStr);

    // generate struct unik dari nama fungsi
    let struct_name = Ident::new(
        &format!("__Hook_{}", fn_name.to_string()),
        fn_name.span(),
    );

    let expanded = quote! {
        #fn_item

        pub struct #struct_name;

        impl omp::events::Events for #struct_name {
            fn #fn_name(&mut self, player: omp::players::Player) {
                #fn_name(player);
            }
        }

        inventory::submit! {
            #struct_name
        }
    };

    TokenStream::from(expanded)
}
