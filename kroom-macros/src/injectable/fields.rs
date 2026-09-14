use syn::{GenericArgument, ItemStruct, PathArguments, Type, parse_quote};
use quote::quote;

pub fn wrap_injected_field_with_arc(item_struct: &mut ItemStruct) {
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

pub fn wrap_multi_injected_field_with_arc(item_struct: &mut ItemStruct) {
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

pub fn extract_inner_type_from_vector(injected_type: &Type) -> Option<&Type> {
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

pub fn get_injected_fields_for_constructor(
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

pub fn get_multi_injected_fields_for_constructor(
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

pub fn strip_attribute_from_struct_fields(attribute_name: &'static str, item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        field.attrs.retain(|attribute| {
            return !attribute.path().is_ident(attribute_name);
        });
    }
}