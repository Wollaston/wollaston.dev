#[cfg(feature = "ssr")]
pub mod ssr {
    // const DB_URL: &str = "sqlite:content.db";
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:content.db").await?)
    }
}
