use proc_macro::TokenStream;
use syn::{ItemStruct, Type, parse_macro_input, parse_quote};
use quote::quote;

use crate::injectable::{args::{MacroArgs, Scope, parse_macro_args}, codegen::generate_injectable, fields::{get_injected_fields_for_constructor, get_multi_injected_fields_for_constructor, strip_attribute_from_struct_fields, wrap_injected_field_with_arc, wrap_multi_injected_field_with_arc}};

mod args;
mod fields;
mod codegen;

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