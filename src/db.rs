use std::io;

use async_graphql::Object;
use async_graphql::SimpleObject;

use sqlx::FromRow;

use sqlx::postgres::PgPool;

/// Database information
#[derive(Debug, SimpleObject, FromRow)]
pub struct DbInfo {
    /// The name of the database.
    pub datname: String,
    /// The character set encoding of the database.
    pub encoding: i32,
    /// The collation of the database.
    pub datcollate: String,
    /// The character classification of the database.
    pub datctype: String,
    /// Whether the database is a template.
    pub datistemplate: bool,
    /// Whether connections are allowed to the database.
    pub datallowconn: bool,
    /// The connection limit for the database.
    pub datconnlimit: i32,
}

pub async fn get_db_info(
    p: &PgPool,
    datname_pattern: Option<String>,
    omit_templates: Option<bool>,
) -> Result<Vec<DbInfo>, io::Error> {
    let datname_pattern = datname_pattern.unwrap_or("%".to_string());
    let omit_templates = omit_templates.unwrap_or(false);

    let rows = sqlx::query_as!(
        DbInfo,
        r#"(
            SELECT
                datname,
                encoding,
                datcollate,
                datctype,
                datistemplate,
                datallowconn,
                datconnlimit
            FROM pg_database
            WHERE
                (datname LIKE $1)
                AND (datistemplate = false OR $2 = false)
        )"#,
        datname_pattern,
        omit_templates,
    )
    .fetch_all(p)
    .await
    .map_err(io::Error::other)?;

    Ok(rows)
}

pub struct DbQuery {
    pool: PgPool,
}

#[Object]
impl DbQuery {
    async fn databases(
        &self,

        #[graphql(desc = "Filter databases by name.")] dbname_pattern: Option<String>,

        #[graphql(desc = "Omit templates.")] omit_templates: Option<bool>,
    ) -> Result<Vec<DbInfo>, io::Error> {
        let p: &PgPool = &self.pool;
        get_db_info(p, dbname_pattern, omit_templates).await
    }
}

pub fn query_new(pool: &PgPool) -> DbQuery {
    DbQuery { pool: pool.clone() }
}
