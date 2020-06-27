#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

mod memory;
mod schedule;

use syn::export::TokenStream;
use syn::{parse_macro_input, AttributeArgs};


/// Create a cron task
///
/// Syntax: `#[crontab(cron= "*/5 * * * * *", desc = "executed every 5 second")]`
///
/// ```
/// #[crontab(cron = "*/5 * * * * *", desc = "每5s执行一次")]
/// fn test1() {
///     println!("test1");
/// }
/// ```
/// Attributes are the same as in [crontab](attr.crontab.html)
#[proc_macro_attribute]
pub fn crontab(attr: TokenStream, item: TokenStream) -> TokenStream {
    schedule::parse_config(parse_macro_input!(attr as AttributeArgs));
    schedule::renew_func_sign(item)
}

/// Create a local cache
///
/// Syntax: `#[local_cache(expire= "60", desc = "update once of every 60 second")]`
///
/// ```
/// #[local_cache(expire = 60, desc = "缓存更新周期60s")]
/// fn test2() {
///     println!("test2");
/// }
/// ```
/// Attributes are the same as in [local_cache](attr.local_cache.html)
#[proc_macro_attribute]
pub fn local_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    memory::parse_config(parse_macro_input!(attr as AttributeArgs));
    memory::renew_func_sign(item)
}
