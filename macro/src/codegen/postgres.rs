use crate::codegen::common::ormlitexCodegen;
use ormlitex_attr::TableMetadata;
use ormlitex_core::query_builder::Placeholder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct PostgresBackend {}

impl ormlitexCodegen for PostgresBackend {
    fn database_ts(&self) -> TokenStream {
        quote! { ::ormlitex::postgres::Postgres }
    }

    fn placeholder_ts(&self) -> TokenStream {
        quote! {
            ::ormlitex::query_builder::Placeholder::dollar_sign()
        }
    }

    fn placeholder(&self) -> Placeholder {
        Placeholder::dollar_sign()
    }

}
