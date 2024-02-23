use leptos::{use_context, ServerFnError};
use sqlx::SqlitePool;

pub async fn db() -> Result<SqlitePool, ServerFnError> {
    Ok(SqlitePool::connect("sqlite:content.db").await?)
}

pub fn pool() -> Result<SqlitePool, ServerFnError> {
    use_context::<SqlitePool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}
