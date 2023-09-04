use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use ormlitex_attr::{Ident, ModelMetadata, TableMetadata};
use crate::codegen::common::{insertion_binding, ormlitexCodegen};
use crate::codegen::insert::impl_Model__insert;
use crate::codegen::select::impl_Model__select;
use crate::codegen::update::impl_Model__update_all_fields;
use crate::MetadataCache;


pub fn impl_Model(db: &dyn ormlitexCodegen, attr: &ModelMetadata, metadata_cache: &MetadataCache) -> TokenStream {
    let model = &attr.inner.struct_name;
    let partial_model = attr.builder_struct();

    let impl_Model__insert = impl_Model__insert(db, &attr.inner, metadata_cache);
    let impl_Model__update_all_fields = impl_Model__update_all_fields(db, attr);
    let impl_Model__delete = impl_Model__delete(db, attr);
    let impl_Model__fetch_one = impl_Model__fetch_one(db, attr);
    let impl_Model__select = impl_Model__select(db, &attr.inner);

    let db = db.database_ts();
    quote! {
        impl ::ormlitex::model::Model<#db> for #model {
            #impl_Model__insert
            #impl_Model__update_all_fields
            #impl_Model__delete
            #impl_Model__fetch_one
            #impl_Model__select

           fn query(query: &str) -> ::ormlitex::query::QueryAs<#db, Self, <#db as ::ormlitex::database::HasArguments>::Arguments> {
                ::ormlitex::query_as::<_, Self>(query)
            }
        }
    }
}

pub fn impl_HasModelBuilder(db: &dyn ormlitexCodegen, attr: &ModelMetadata) -> TokenStream {
    let model = &attr.inner.struct_name;
    let partial_model = attr.builder_struct();

    let impl_Model__builder = impl_Model__builder(attr);
    let impl_Model__update_partial = impl_Model__update_partial(attr);

    let db = db.database_ts();
    quote! {
        impl<'slf> ::ormlitex::model::HasModelBuilder<'slf, #db> for #model {
            type ModelBuilder = #partial_model<'slf>;

            #impl_Model__builder
            #impl_Model__update_partial
        }
    }
}

pub fn impl_Model__delete(db: &dyn ormlitexCodegen, attr: &ModelMetadata) -> TokenStream {
    let mut placeholder = db.placeholder();

    let query = format!(
        "DELETE FROM \"{}\" WHERE {} = {}",
        attr.inner.table_name,
        attr.pkey.column_name,
        placeholder.next().unwrap()
    );

    let box_future = crate::util::box_fut_ts();
    let db = db.database_ts();
    let id = &attr.pkey.identifier;
    quote! {
        fn delete<'e, E>(self, db: E) -> #box_future<'e, ::ormlitex::Result<()>>
        where
            E: 'e +::ormlitex::Executor<'e, Database = #db>
        {
            Box::pin(async move {
                let row =::ormlitex::query(#query)
                    .bind(self.#id)
                    .execute(db)
                    .await
                    .map_err(::ormlitex::Error::from)?;
                if row.rows_affected() == 0 {
                    Err(::ormlitex::Error::from(::ormlitex::SqlxError::RowNotFound))
                } else {
                    Ok(())
                }
            })
        }
    }
}


pub fn impl_Model__fetch_one(db: &dyn ormlitexCodegen, attr: &ModelMetadata) -> TokenStream {
    let mut placeholder = db.placeholder();

    let query = format!(
        "SELECT * FROM \"{}\" WHERE {} = {}",
        attr.inner.table_name,
        attr.pkey.column_name,
        placeholder.next().unwrap()
    );

    let db = db.database_ts();
    let box_future = crate::util::box_fut_ts();
    quote! {
        fn fetch_one<'e, 'a, Arg, E>(id: Arg, db: E) -> #box_future<'e, ::ormlitex::Result<Self>>
        where
            'a: 'e,
            Arg: 'a + Send + ::ormlitex::Encode<'a, #db> + ::ormlitex::types::Type<#db>,
            E: 'e +::ormlitex::Executor<'e, Database = #db>
        {
            Box::pin(async move {
                ::ormlitex::query_as::<#db, Self>(#query)
                    .bind(id)
                    .fetch_one(db)
                    .await
                    .map_err(::ormlitex::Error::from)
            })
        }
    }
}


pub fn impl_Model__builder(attr: &ModelMetadata) -> TokenStream {
    let partial_model = &attr.builder_struct();
    quote! {
        fn builder() -> #partial_model<'static> {
            #partial_model::default()
        }
    }
}

pub fn impl_Model__update_partial(attr: &ModelMetadata) -> TokenStream {
    let partial_model = &attr.builder_struct();
    quote! {
        fn update_partial(&'slf self) -> #partial_model<'slf> {
            let mut partial = #partial_model::default();
            partial.updating = Some(&self);
            partial
        }
    }
}
