use axum::extract::FromRef;
use leptos::config::{get_configuration, LeptosOptions};
use std::borrow::Cow;
use std::env;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

use crate::error_template::AppError;

/// Holds the environment state for the backend
#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_path: Cow<'static, str>,
}

impl EnvironmentVariables {
    /// Creates a new EnvironmentVariables struct from
    /// the environment, or with a default value if one
    /// is not provided from the environment.
    fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_path: match env::var("DATABASE_PATH") {
                Ok(path) => path.into(),
                _ => "data".into(),
            },
        })
    }
}

/// Holds the state for the server that will be passed to the handlers
/// Includes:
///     -   DB connection
///     -   Environment Variables
#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Surreal<surrealdb::engine::local::Db>,
    pub env: EnvironmentVariables,
}

impl AppState {
    /// Initializes and creats a new AppState instance
    pub async fn new() -> Result<AppState, AppError> {
        let env = EnvironmentVariables::from_env()?;

        let conf = get_configuration(None)?;
        let leptos_options = conf.leptos_options;

        Ok(AppState {
            leptos_options,
            db: AppState::init_db(&env).await?,
            env,
        })
    }

    /// Initializes a DB connection, and configures the DB if it doesn't already exist.
    async fn init_db(env: &EnvironmentVariables) -> Result<Surreal<Db>, AppError> {
        let db = Surreal::new::<RocksDb>(&*env.database_path)
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        // Init the namespace and database if
        // they do not already exist
        db.query(
            "
DEFINE NAMESPACE IF NOT EXISTS wollaston;
USE NS wollaston;

DEFINE DATABASE IF NOT EXISTS wollaston;
;",
        )
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        // Sign into the namespace and database for this project
        db.use_ns("wollaston")
            .use_db("wollaston")
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        // Initialize basic db if it doesn't exist already.
        // This is useful when launching the project on a new machine
        // or when refreshing a project.
        db.query(
            "
DEFINE TABLE IF NOT EXISTS blogs SCHEMALESS;
DEFINE FIELD created ON TABLE blogs TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD updated ON TABLE blogs VALUE time::now();

DEFINE TABLE IF NOT EXISTS projects SCHEMALESS;
DEFINE FIELD created ON TABLE projects TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD updated ON TABLE projects VALUE time::now();
",
        )
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        Ok(db)
    }
}
