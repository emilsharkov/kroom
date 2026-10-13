use kroom_core::scope::Scope;
use proc_macro::TokenStream;
use std::str::FromStr;
use syn::{LitStr, Type, meta, parse::Parser};
use quote::quote;

pub struct MacroArgs {
    pub target_type: Option<Type>,
    pub scope: Type,
}

impl MacroArgs {
    pub(crate) fn new() -> Self {
        MacroArgs { 
            target_type: None,
            scope: syn::parse2(quote!{ ::kroom_core::scope::Scope::Singleton }).expect("")
        }
    }
}

pub fn parse_macro_args(attr: TokenStream) -> Result<MacroArgs, syn::Error> {
    let mut args = MacroArgs::new();
    let parser = meta::parser(|metadata| {
        if metadata.path.is_ident("scope") {
            let metadata_value = metadata.value()?;
            if metadata_value.peek(LitStr) {
                let string_value: String = metadata_value.parse::<LitStr>()?.value();
                let scope = Scope::from_str(&string_value).map_err(|err| metadata.error(err))?;
                let scope_type: Type = match scope {
                    Scope::Singleton => syn::parse_quote!(::kroom_core::scope::Scope::Singleton),
                    Scope::Transient => syn::parse_quote!(::kroom_core::scope::Scope::Transient),
                };
                args.scope = scope_type;
            } else {
                let scope_type: Type = metadata_value.parse::<Type>()?;
                args.scope = scope_type;
            }
            Ok(())
        } else if metadata.path.is_ident("type") {
            let target_type: Type = metadata.value()?.parse::<Type>()?;
            args.target_type = Some(target_type);
            Ok(())
        } else {
            Err(metadata.error("Unsupported macro argument"))
        }
    });
    parser.parse(attr.into())?;
    Ok(args)
}
