pub mod db;
pub mod schema;

use std::io;

use async_graphql::MergedObject;

use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;

use sqlx::postgres::PgPool;

use crate::db::DbQuery;
use crate::schema::SchemaQuery;

#[derive(MergedObject)]
pub struct Query(pub DbQuery, pub SchemaQuery);

pub fn pool2query(pool: PgPool) -> Query {
    let db_query = db::query_new(&pool);
    let schema_query = schema::query_new(&pool);

    Query(db_query, schema_query)
}

pub async fn pool_new(conn_str: &str) -> Result<PgPool, io::Error> {
    PgPool::connect(conn_str).await.map_err(io::Error::other)
}

pub async fn query_new(conn_str: &str) -> Result<Query, io::Error> {
    let pool = pool_new(conn_str).await?;
    let query = pool2query(pool);
    Ok(query)
}

pub type PgSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn query2schema(q: Query) -> PgSchema {
    Schema::build(q, EmptyMutation, EmptySubscription).finish()
}

pub async fn conn2schema(conn_str: &str) -> Result<PgSchema, io::Error> {
    let query = query_new(conn_str).await?;
    let schema = query2schema(query);
    Ok(schema)
}
