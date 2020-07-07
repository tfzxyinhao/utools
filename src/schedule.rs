extern crate quote;
extern crate proc_macro;

use quote::quote;
use syn::{AttributeArgs, Ident};
use self::quote::ToTokens;
use syn::export::{TokenStream, Span};

fn append_schedule_item(fun_name: String, wrap_name: String, cron: String, desc: String) {
    println!("append_schedule_item fun_name={} wrap_name={} cron={}, desc={}", fun_name, wrap_name, cron, desc)
}

pub fn parse_config(fun_name: String, wrap_name: String, args: AttributeArgs) {
    let mut cron: String = "".to_string();
    let mut desc: String = "".to_string();

    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                if nv.path.is_ident("cron") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        cron = lit.value();
                    } else {
                        println!("expire = error")
                    };
                } else if nv.path.is_ident("desc") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        desc = lit.value();
                    } else {
                        println!("desc = error")
                    };
                }
            }
            _ => ()
        }
    }

    if cron.len() > 0 && desc.len() > 0 {
        append_schedule_item(fun_name, wrap_name, cron, desc);
    }
}

pub fn renew_func_sign(item: TokenStream) -> (String, String, TokenStream) {
    let ast: syn::ItemFn = syn::parse(item).unwrap();
    let name = ast.sig.ident.clone();

    let origin_name = name.to_string();
    let wrap_name = name.to_string() + "_wrap";
    let wrap_fun_name = Ident::new(&wrap_name, Span::call_site());

    let mut out = syn::export::TokenStream2::new();

    let tokens = quote! {
           fn #wrap_fun_name() {
                println!("name test2");
            }
    };

    ast.to_tokens(&mut out);
    tokens.to_tokens(&mut out);
    (origin_name, wrap_name, out.to_token_stream().into())
}
