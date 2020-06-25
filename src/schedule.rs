extern crate proc_macro;

use syn::export::TokenStream;
use syn::{AttributeArgs};

fn append_schedule_item(cron: String, desc: String) {
    println!("append_schedule_item expire={}, desc={}", cron, desc)
}

pub fn parse_config(args: AttributeArgs) {
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
        append_schedule_item(cron, desc);
    }
}

pub fn renew_func_sign(item: TokenStream) -> TokenStream {
    item
}
