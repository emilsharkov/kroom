use syn::Type;
use quote::quote;

pub fn generate_injectable(
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

            fn __kroom_scope() -> kroom_core::scope::Scope {
                ::kroom_core::scope::Scope::Singleton
            }

            fn __kroom_dependent_types() -> Vec<std::any::TypeId> {
                vec![
                    ::std::any::TypeId::of::<dyn UserRepo>()
                ]
            }
        }

        ::kroom_core::inventory::submit! {
            ::kroom_core::registration::Registration::of::<#target_type, #struct_type>()
        }
    )
}