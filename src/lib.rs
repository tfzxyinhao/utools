#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

mod memory;
mod schedule;

use syn::export::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[proc_macro_attribute]
pub fn crontab(attr: TokenStream, item: TokenStream) -> TokenStream {
    schedule::parse_config(parse_macro_input!(attr as AttributeArgs));
    schedule::renew_func_sign(item)
}

#[proc_macro_attribute]
pub fn local_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    memory::parse_config(parse_macro_input!(attr as AttributeArgs));
    memory::renew_func_sign(item)
}
