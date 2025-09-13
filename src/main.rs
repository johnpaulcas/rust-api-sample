use axum::{Router, routing::get};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- env -- //
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let api_url = std::env::var("API_URL").expect("API_URL");

    // -- db -- //
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(20)
        .max_connections(50)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // -- api -- //
    let listener = tokio::net::TcpListener::bind(&api_url).await?;
    axum::serve(listener, router()).await?;

    Ok(())
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "welcome to rust api sample"
}
