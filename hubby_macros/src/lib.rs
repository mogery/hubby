#[macro_use]
extern crate lazy_static;

use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};

use proc_macro2::Span;
use proc_macro::{TokenStream};
use quote::{quote};
use syn::{self, parse_macro_input, parse::{Parse, ParseStream}, Ident, LitInt, ItemFn, ItemStruct};

struct RegisteredPacket {
    id: i32,
    name: String,
}

type PacketRegistry = Vec<RegisteredPacket>;

lazy_static! {
    static ref HANDSHAKING_REGISTRY: RwLock<PacketRegistry> = RwLock::new(Vec::new());
    static ref STATUS_REGISTRY: RwLock<PacketRegistry> = RwLock::new(Vec::new());
    static ref LOGIN_REGISTRY: RwLock<PacketRegistry> = RwLock::new(Vec::new());
    static ref PLAY_REGISTRY: RwLock<PacketRegistry> = RwLock::new(Vec::new());
}

struct RegisterPacketArgs {
    id: i32
}

impl Parse for RegisterPacketArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id = LitInt::parse(input)?;
        Ok(RegisterPacketArgs {
            id: id.base10_parse()?,
        })
    }
}

fn register_generic_packet(mut reg: RwLockWriteGuard<PacketRegistry>, args: TokenStream, input: TokenStream) -> TokenStream {
    let ic = input.clone();
    let input_item = parse_macro_input!(ic as ItemFn);
    let args = parse_macro_input!(args as RegisterPacketArgs);

    reg.push(RegisteredPacket {
        id: args.id,
        name: input_item.sig.ident.to_string(),
    });

    input
}

#[proc_macro_attribute]
pub fn register_handshaking_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    register_generic_packet(HANDSHAKING_REGISTRY.write().unwrap(), args, input)
}

#[proc_macro_attribute]
pub fn register_status_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    register_generic_packet(STATUS_REGISTRY.write().unwrap(), args, input)
}

#[proc_macro_attribute]
pub fn register_login_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    register_generic_packet(LOGIN_REGISTRY.write().unwrap(), args, input)
}

#[proc_macro_attribute]
pub fn register_play_packet(args: TokenStream, input: TokenStream) -> TokenStream {
    register_generic_packet(PLAY_REGISTRY.write().unwrap(), args, input)
}

// GENERATORS

fn generate_generic_handler(reg: RwLockReadGuard<PacketRegistry>, _input: TokenStream) -> TokenStream {
    let ids: Vec<i32> = reg.iter().map(|x| x.id).collect();
    let funcs: Vec<Ident> = reg.iter().map(|x| syn::Ident::new(&x.name, Span::call_site())).collect();

    TokenStream::from(quote!(
        pub async fn handle(conn: &mut Connection<'_>, id: i32, data: &[u8]) -> Result<(), HandleError> {
            match id {
                #(#ids => #funcs(conn, serde_mcje::from_slice(data).map_err(HandleError::SerdeMCJE)?).await,)*
                _ => Err(HandleError::Unimplemented(id))
            }
        }
    ))
}

#[proc_macro]
pub fn generate_handshaking_handler(input: TokenStream) -> TokenStream {
    generate_generic_handler(HANDSHAKING_REGISTRY.read().unwrap(), input)
}

#[proc_macro]
pub fn generate_status_handler(input: TokenStream) -> TokenStream {
    generate_generic_handler(STATUS_REGISTRY.read().unwrap(), input)
}

#[proc_macro]
pub fn generate_login_handler(input: TokenStream) -> TokenStream {
    generate_generic_handler(LOGIN_REGISTRY.read().unwrap(), input)
}

#[proc_macro]
pub fn generate_play_handler(input: TokenStream) -> TokenStream {
    generate_generic_handler(PLAY_REGISTRY.read().unwrap(), input)
}

#[proc_macro_attribute]
pub fn identify_packet(args: TokenStream, mut input: TokenStream) -> TokenStream {
    let ic = input.clone();
    let input_item = parse_macro_input!(ic as ItemStruct);
    let args = parse_macro_input!(args as RegisterPacketArgs);

    let sn = input_item.ident;
    let id = args.id;

    let imp = TokenStream::from(quote! {
        impl IdentifiedPacket for #sn {
            const ID: i32 = #id;
        }
    });
    input.extend(imp);

    input
}

