use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::SqlitePool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
}
