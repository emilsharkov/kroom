use std::str::FromStr;
use quote::quote;
use syn::{GenericArgument, ItemStruct, LitStr, PathArguments, Type, meta, parse::Parser, parse_macro_input, parse_quote};
use proc_macro::{TokenStream,};

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
    let args: MacroArgs = parse_macro_args(attr)
        .unwrap_or(MacroArgs::default());
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &item_struct.ident;
    
    let struct_type: Type = parse_quote!(#struct_ident);
    let target_type: Type = args.target_type.unwrap_or_else(|| {
        parse_quote!(#struct_ident)
    });
    let _scope = args.scope.unwrap_or(Scope::Singleton);

    let injected_fields = get_injected_fields_for_constructor(&item_struct);
    let multi_injected_fields = get_multi_injected_fields_for_constructor(&item_struct);
    let target_type_injectable = generate_injectable(&target_type, &struct_type, &injected_fields, &multi_injected_fields);
    let struct_type_injectable = match target_type != struct_type {
        true => Some(generate_injectable(&struct_type, &struct_type, &injected_fields, &multi_injected_fields)),
        false => None
    };

    wrap_injected_field_with_arc(&mut item_struct);
    wrap_multi_injected_field_with_arc(&mut item_struct);

    // strip inject/multi_inject from struct
    strip_attribute_from_struct_fields("inject", &mut item_struct);
    strip_attribute_from_struct_fields("multi_inject", &mut item_struct);

    let output = quote!(
        #item_struct

        #target_type_injectable
        #struct_type_injectable
    );
    output.into()
}

fn wrap_injected_field_with_arc(item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        let field_has_inject_attribute = field.attrs.iter().any(|attribute| {
            return attribute.meta.path().is_ident("inject");
        });
        if field_has_inject_attribute {
            let injected_type = &field.ty;
            field.ty = parse_quote!(
                ::std::sync::Arc<#injected_type>
            );
        }
    }
}

fn wrap_multi_injected_field_with_arc(item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        let field_has_inject_attribute = field.attrs.iter().any(|attribute| {
            return attribute.meta.path().is_ident("multi_inject");
        });
        if field_has_inject_attribute {
            let injected_type = &field.ty;
            let inner_type = extract_inner_type_from_vector(injected_type);
            field.ty = parse_quote!(
                Vec<::std::sync::Arc<#inner_type>>
            );
        }
    }
}

fn extract_inner_type_from_vector(injected_type: &Type) -> Option<&Type> {
    let Type::Path(type_path) = injected_type else {
        return None;
    };

    let last_segment = type_path.path.segments.last()?;
    if last_segment.ident != "Vec" {
        return None;
    }

    let PathArguments::AngleBracketed(argument) = &last_segment.arguments else {
        return None;
    };
    argument.args.iter().find_map(|arg| match arg {
        GenericArgument::Type(inner_type) => Some(inner_type),
        _ => None
    })
}

fn strip_attribute_from_struct_fields(attribute_name: &'static str, item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        field.attrs.retain(|attribute| {
            return !attribute.path().is_ident(attribute_name);
        });
    }
}

fn generate_injectable(
    target_type: &Type,
    struct_type: &Type,
    injected_fields: &Vec<proc_macro2::TokenStream>,
    multi_injected_fields: &Vec<proc_macro2::TokenStream>
) -> proc_macro2::TokenStream {
    quote!(
        impl ::kroom_core::injectable::Injectable<#target_type> for #struct_type {
            fn __kroom_construct(_container: &::kroom_core::container::Container) -> ::std::sync::Arc<#target_type> {
                ::std::sync::Arc::new(#struct_type {
                    #(#injected_fields)*
                    #(#multi_injected_fields)*
                })
            }
        }

        ::kroom_core::inventory::submit! {
            ::kroom_core::registration::Registration::of::<#target_type, #struct_type>()
        }
    )
}

fn parse_macro_args(
    attr: TokenStream,
) -> Result<MacroArgs, syn::Error> {
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
    parser.parse(attr.into())?;
    Ok(args)
}

fn get_injected_fields_for_constructor(
    item_struct: &ItemStruct
) -> Vec<proc_macro2::TokenStream> {
    let mut injected_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    for field in item_struct.fields.iter() {
        let field_name = field.ident.as_ref().expect("Field name to exist");
        let field_has_inject_attribute = field.attrs.iter().any(|attribute| {
            return attribute.meta.path().is_ident("inject");
        });
        if field_has_inject_attribute {
            let injected_type = &field.ty;
            injected_fields.push(quote!(
                #field_name: _container.get::<#injected_type>(),
            ))
        }
    }
    return injected_fields;
}

fn get_multi_injected_fields_for_constructor(
    item_struct: &ItemStruct
) -> Vec<proc_macro2::TokenStream> {
    let mut injected_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    for field in item_struct.fields.iter() {
        let field_name = field.ident.as_ref().expect("Field name to exist");
        let field_has_inject_attribute = field.attrs.iter().any(|attribute| {
            return attribute.meta.path().is_ident("multi_inject");
        });
        if field_has_inject_attribute {
            let injected_type = &field.ty;
            let inner_type = extract_inner_type_from_vector(injected_type);
            injected_fields.push(quote!(
                #field_name: _container.get_all::<#inner_type>(),
            ))
        }
    }
    return injected_fields;
}