use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;
use sqlx::Sqlite;

pub async fn create_pool(database_url: &str) -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool.")
}
