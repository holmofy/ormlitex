use crate::codegen::common::ormlitexCodegen;

use ormlitex_core::query_builder::Placeholder;
use proc_macro2::TokenStream;
use quote::quote;

pub struct SqliteBackend {}

impl ormlitexCodegen for SqliteBackend {
    fn database_ts(&self) -> TokenStream {
        quote! { ::ormlitex::sqlite::Sqlite }
    }

    fn placeholder_ts(&self) -> TokenStream {
        quote! {
            ::ormlitex::query_builder::Placeholder::question_mark()
        }
    }

    fn placeholder(&self) -> Placeholder {
        Placeholder::question_mark()
    }
}
