extern crate quote;
extern crate proc_macro;

use quote::quote;

use std::sync::RwLock;
use syn::{AttributeArgs, Ident};
use std::collections::HashSet;
use self::quote::ToTokens;
use syn::export::{TokenStream, Span};

lazy_static! {
    static ref _CONTAINER: RwLock<MemoryCacheContainer> = RwLock::new(MemoryCacheContainer::new());
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MemoryCacheItem {
    origin_fun_name: String,
    wrap_fun_name: String,
    call_count: u64,
    avg_execute_duration: u32,
    expire: u32,
    desc: String,
}

impl MemoryCacheItem {
    fn new(origin_fun_name: String, wrap_fun_name: String, expire: u32, desc: String) -> MemoryCacheItem {
        MemoryCacheItem {
            origin_fun_name,
            wrap_fun_name,
            call_count: 0,
            avg_execute_duration: 0,
            expire,
            desc,
        }
    }
}

#[derive(Clone, Debug)]
struct MemoryCacheContainer {
    version: u32,
    item_list: HashSet<MemoryCacheItem>,
}

impl<'a> MemoryCacheContainer {
    pub fn new() -> Self {
        let mut r = MemoryCacheContainer {
            version: rand::random::<u32>(),
            item_list: HashSet::new(),
        };

        r.start();

        r
    }

    pub fn add(&mut self, _item: MemoryCacheItem) {
        self.item_list.insert(_item);
    }

    fn start(&mut self) {
        println!("{}", self.item_list.len())
    }
}

fn append_cache_item(origin_name: String, wrap_name: String, expire: u32, desc: String) {
    println!("append_cache_item expire={}, desc={}", expire, desc);
    match _CONTAINER.write() {
        Ok(mut container) => {
            container.add(MemoryCacheItem::new(origin_name, wrap_name, expire, desc));
        }
        _ => ()
    }
}

pub fn parse_config(origin_name: String, wrap_name: String, args: AttributeArgs) {
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
        append_cache_item(origin_name, wrap_name, expire, desc);
    }
}

pub fn renew_func_sign(item: TokenStream) -> (String, String, TokenStream) {
    let mut ast: syn::ItemFn = syn::parse(item).unwrap();
    let name = ast.sig.ident.clone();

    let origin_name = name.to_string();
    let wrap_name = name.to_string() + "_wrap";
    let wrap_fun_name = Ident::new(&*wrap_name, Span::call_site());

    ast.sig.ident = wrap_fun_name;
    let mut out = syn::export::TokenStream2::new();

    let tokens = quote! {
           fn #name() {
                println!("name test2");
            }
    };

    ast.to_tokens(&mut out);
    tokens.to_tokens(&mut out);
    (origin_name, wrap_name, out.to_token_stream().into())
}


