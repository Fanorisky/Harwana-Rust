use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Meta, Expr, punctuated::Punctuated, Token};

struct MetaList {
    metas: Punctuated<Meta, Token![,]>,
}

impl syn::parse::Parse for MetaList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            metas: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // default name = nama fungsi
    let mut names: Vec<String> = vec![fn_name.to_string()];

    if !attr.is_empty() {
        let meta_list = parse_macro_input!(attr as MetaList);
        for meta in meta_list.metas {
            match meta {
                Meta::Path(path) => {
                    if let Some(ident) = path.get_ident() {
                        names[0] = ident.to_string();
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("name") => {
                    if let Expr::Lit(expr_lit) = nv.value {
                        if let syn::Lit::Str(lit) = expr_lit.lit {
                            names[0] = lit.value();
                        }
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("alias") => {
                    if let Expr::Array(arr) = nv.value {
                        for elem in arr.elems {
                            if let Expr::Lit(expr_lit) = elem {
                                if let syn::Lit::Str(lit) = expr_lit.lit {
                                    names.push(lit.value());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let reg_fn_name = format_ident!("register_{}", fn_name);
    let names_ref: Vec<_> = names.iter().map(|n| n.as_str()).collect();

    let gen = quote! {
        #vis #sig #block

        #[ctor::ctor]
        fn #reg_fn_name() {
            crate::command::processor::register_command(&[#(#names_ref),*], #fn_name);
        }
    };

    gen.into()
}
