use proc_macro2::TokenStream;
use quote::quote;
use ormlitex_attr::TableMetadata;
use crate::codegen::common::ormlitexCodegen;

/// Allows the model to be turned into arguments. This can be used for bulk insertion.
pub fn impl_IntoArguments(db: &dyn ormlitexCodegen, attr: &TableMetadata) -> TokenStream {
    let mut placeholder = db.placeholder();
    let db = db.database_ts();
    let model = &attr.struct_name;
    let params = attr.database_columns()
        .map(|c| {
            let field = &c.identifier;
            let value = if c.is_json() || c.experimental_encode_as_json {
                quote! {
                    ::ormlitex::types::Json(self.#field)
                }
            } else {
                quote! {
                    self.#field
                }
            };
            quote! {
                ::ormlitex::Arguments::add(&mut args, #value);
            }
        });

    quote! {
        impl<'a> ::ormlitex::IntoArguments<'a, #db> for #model {
            fn into_arguments(self) -> <#db as ::ormlitex::database::HasArguments<'a>>::Arguments {
                let mut args = <#db as ::ormlitex::database::HasArguments<'a>>::Arguments::default();
                #(
                    #params
                )*
                args
            }
        }
    }
}