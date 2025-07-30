use std::io;
use std::process::ExitCode;

use axum::Router;

use tokio::net::TcpListener;

use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;

use rs_pg_schemas2ql::PgSchema;
use rs_pg_schemas2ql::conn2schema;

async fn schema_new(conn_str: &str) -> Result<PgSchema, io::Error> {
    conn2schema(conn_str).await
}

async fn req2res(s: &PgSchema, req: GraphQLRequest) -> GraphQLResponse {
    s.execute(req.into_inner()).await.into()
}

fn env2conn_str() -> Result<String, io::Error> {
    std::env::var("DATABASE_URL").map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "DATABASE_URL environment variable not found",
        )
    })
}

fn env2listen_addr() -> Result<String, io::Error> {
    std::env::var("LISTEN_ADDR").map_err(io::Error::other)
}

async fn sub() -> Result<(), io::Error> {
    let conn: String = env2conn_str()?;
    let ladr: String = env2listen_addr()?;

    let s: PgSchema = schema_new(&conn).await?;
    let sdl: String = s.sdl();
    std::fs::write("pg-schemas.graphql", sdl.as_bytes())?;

    let app = Router::new().route(
        "/",
        axum::routing::post(|req: GraphQLRequest| async move { req2res(&s, req).await }),
    );

    let lis = TcpListener::bind(ladr).await?;

    axum::serve(lis, app).await
}

#[tokio::main]
async fn main() -> ExitCode {
    sub().await.map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    })
}
