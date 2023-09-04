use proc_macro2::TokenStream;
use quote::quote;
use ormlitex_attr::TableMetadata;
use crate::codegen::common::ormlitexCodegen;

pub fn impl_Model__select(db: &dyn ormlitexCodegen, attr: &TableMetadata) -> TokenStream {
    let table_name = &attr.table_name;
    let db = db.database_ts();
    quote! {
        fn select<'args>() -> ::ormlitex::query_builder::SelectQueryBuilder<'args, #db, Self> {
            ::ormlitex::query_builder::SelectQueryBuilder::default()
                .select(format!("\"{}\".*", #table_name))
        }
    }
}