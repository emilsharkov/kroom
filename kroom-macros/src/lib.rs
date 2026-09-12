use std::str::FromStr;
use quote::quote;
use syn::{ItemStruct, LitStr, Type, meta, parse_macro_input, parse_quote};
use proc_macro::TokenStream;

enum Scope {
    Singleton,
    Transient,
    Scoped
}
impl FromStr for Scope {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "singleton" => Ok(Scope::Singleton),
            "transient" => Ok(Scope::Transient),
            "scoped" => Ok(Scope::Scoped),
            _ => Err(format!("{} is not a valid Scope",s)),
        }
    }
}

#[derive(Default)]
struct MacroArgs {
    target_type: Option<Type>,
    scope: Option<Scope>
}

#[proc_macro_attribute]
pub fn injectable(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let mut args = MacroArgs::default();
    let parser = meta::parser(|metadata| {
        if metadata.path.is_ident("scope") {
            let value: String = metadata
                .value()?
                .parse::<LitStr>()?
                .value();
            let scope = Scope::from_str(&value)
                .map_err(|err| metadata.error(err))?;
            args.scope = Some(scope);
            Ok(())
        } else if metadata.path.is_ident("type") {
            let target_type: Type = metadata
                .value()?
                .parse()?;
            args.target_type = Some(target_type);
            Ok(())
        } else {
            Err(metadata.error("Unsupported macro argument"))
        }
    });
    parse_macro_input!(attr with parser);

    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &item_struct.ident;
    
    let struct_type: Type = parse_quote!(#struct_ident);
    let target_type: Type = args.target_type.unwrap_or_else(|| {
        parse_quote!(#struct_ident)
    });
    let _scope = args.scope.unwrap_or(Scope::Singleton);

    let target_type_injectable = generate_injectable(&target_type, &struct_type);
    let struct_type_injectable = if target_type != struct_type {
        Some(generate_injectable(&struct_type, &struct_type))
    } else {
        None
    };
    let output = quote!(
        #item_struct

        #target_type_injectable
        #struct_type_injectable
    );
    output.into()
}

fn generate_injectable(
    target_type: &Type,
    struct_type: &Type,
) -> proc_macro2::TokenStream {
    quote!(
        impl ::kroom_core::injectable::Injectable<#target_type> for #struct_type {
            fn __kroom_construct(_container: &::kroom_core::container::Container) -> ::std::sync::Arc<#target_type> {
                ::std::sync::Arc::new(#struct_type {})
            }
        }

        ::kroom_core::inventory::submit! {
            ::kroom_core::registration::Registration::of::<#target_type, #struct_type>()
        }
    )
}