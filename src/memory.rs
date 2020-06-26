extern crate proc_macro;

use syn::export::TokenStream;
use syn::{AttributeArgs};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct MemoryCacheItem {
    origin_fun_name: String,
    call_count: u64,
    avg_execute_duration: u32,
    expire: u32,
    desc: String,
}

impl MemoryCacheItem {
    fn new(fun_name: String, expire: u32, desc: String) -> MemoryCacheItem {
        MemoryCacheItem {
            origin_fun_name: fun_name,
            call_count: 0,
            avg_execute_duration: 0,
            expire,
            desc,
        }
    }
}

#[derive(Clone, Debug)]
struct MemoryCacheContainer {
    pub version: u32,
    item_list: HashSet<MemoryCacheItem>,
}

impl<'a> MemoryCacheContainer {
    #[warn(dead_code)]
    pub fn new() -> MemoryCacheContainer {
        MemoryCacheContainer {
            version: rand::random::<u32>(),
            item_list: HashSet::new(),
        }
    }

    pub fn add(&mut self, _item: MemoryCacheItem) {
        // self.item_list.insert(item);
    }
}

static mut CONTAINER: Option<MemoryCacheContainer> = None;

fn append_cache_item(expire: u32, desc: String) {
    unsafe {
        if CONTAINER.is_none() {
            CONTAINER = Some(MemoryCacheContainer::new())
        }
    }

    // unsafe { CONTAINER.add(MemoryCacheItem::new("test".to_string(), expire, desc)); }
    println!("append_cache_item expire={}, desc={}", expire, desc)
}

pub fn parse_config(args: AttributeArgs) {
    let mut expire: u32 = 0;
    let mut desc: String = "".to_string();
    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                if nv.path.is_ident("expire") {
                    if let syn::Lit::Int(lit) = nv.lit {
                        expire = lit.base10_parse::<u32>().unwrap();
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

    if expire > 0 && desc.len() > 0 {
        append_cache_item(expire, desc);
    }
}

pub fn renew_func_sign(item: TokenStream) -> TokenStream {
    item
}


