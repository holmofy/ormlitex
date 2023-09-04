#![cfg_attr(docsrs, feature(doc_cfg))]
pub use ormlitex_core::BoxFuture;
pub use ormlitex_core::{Error, Result};
pub use model::{Model, FromRow, TableMeta, IntoArguments};
pub use ::sqlx::{Row, ColumnIndex, Decode, Column, Database};

pub use ::sqlx::{query, query_as, query_as_with, Connection, Executor, Pool, Acquire, ConnectOptions, Encode, Arguments, query_with};
pub use ::sqlx::pool::PoolOptions;

pub mod model;

pub mod query_builder {
    pub use ormlitex_core::query_builder::{SelectQueryBuilder, Placeholder, QueryBuilderArgs};
    pub use ormlitex_core::insert::OnConflict;
}

pub mod types {
    pub use sqlx::types::*;
    pub use ormlitex_macro::ManualType;
}

pub mod decode {
    pub use sqlx::decode::*;
}

pub use sqlx::{Error as SqlxError};

pub mod database {
    pub use sqlx::database::*;
}

/// We need objects available for proc-macros that aren't meant to be available to end users. This module does that.
#[doc(hidden)]
pub mod __private {
    pub use ormlitex_core::join::{JoinDescription, SemanticJoinType};
    pub use ormlitex_core::insert::Insertion;
    pub use sqlmo::Insert;
    pub use tokio_stream::StreamExt;
}

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres {
    pub use sqlx::postgres::*;
}

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub mod sqlite {
    pub use sqlx::sqlite::*;
}