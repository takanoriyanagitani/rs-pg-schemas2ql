use std::io;

use async_graphql::Object;
use async_graphql::SimpleObject;

use sqlx::FromRow;

use sqlx::postgres::PgPool;

/// Represents a PostgreSQL schema.
#[derive(Debug, SimpleObject, FromRow)]
pub struct Schema {
    /// The name of the catalog.
    pub catalog_name: Option<String>,
    /// The name of the schema.
    pub schema_name: Option<String>,
    /// The owner of the schema.
    pub schema_owner: Option<String>,
    /// The default character set catalog.
    pub default_character_set_catalog: Option<String>,
    /// The default character set schema.
    pub default_character_set_schema: Option<String>,
    /// The default character set name.
    pub default_character_set_name: Option<String>,
    /// The SQL path.
    pub sql_path: Option<String>,
}

/// Retrieves schema information from the PostgreSQL database.
pub async fn get_schemas(
    p: &PgPool,
    schema_name_pattern: Option<String>,
) -> Result<Vec<Schema>, io::Error> {
    let schema_name_pattern = schema_name_pattern.unwrap_or("%".to_string());

    let rows = sqlx::query_as!(
        Schema,
        r#"
            SELECT
                catalog_name,
                schema_name,
                schema_owner,
                default_character_set_catalog,
                default_character_set_schema,
                default_character_set_name,
                sql_path
            FROM information_schema.schemata
            WHERE
                schema_name LIKE $1
            ORDER BY schema_name
        "#,
        schema_name_pattern,
    )
    .fetch_all(p)
    .await
    .map_err(io::Error::other)?;

    Ok(rows)
}

pub struct SchemaQuery {
    pool: PgPool,
}

#[Object]
impl SchemaQuery {
    /// Lists schemas in the database.
    async fn schemas(
        &self,
        #[graphql(desc = "Filter schemas by name.")] schema_name_pattern: Option<String>,
    ) -> Result<Vec<Schema>, io::Error> {
        let p: &PgPool = &self.pool;
        get_schemas(p, schema_name_pattern).await
    }
}

pub fn query_new(pool: &PgPool) -> SchemaQuery {
    SchemaQuery { pool: pool.clone() }
}
