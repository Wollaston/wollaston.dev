use axum::extract::FromRef;
use leptos::config::{get_configuration, LeptosOptions};
use std::borrow::Cow;
use std::env;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::error_template::AppError;

/// Holds the environment state for the backend
#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_path: Cow<'static, str>,
    pub database_user: Cow<'static, str>,
    pub database_password: Cow<'static, str>,
}

impl EnvironmentVariables {
    /// Creates a new EnvironmentVariables struct from
    /// the environment, or with a default value if one
    /// is not provided from the environment.
    ///
    /// Panics if the CDG_TOKEN environment variable is
    /// not provided.
    fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_path: match env::var("DATABASE_PATH") {
                Ok(path) => path.into(),
                _ => "data".into(),
            },
            database_user: match env::var("DATABASE_USER") {
                Ok(user) => user.into(),
                _ => "root".into(),
            },
            database_password: match env::var("DATABASE_PASSWORD") {
                Ok(password) => password.into(),
                _ => "root".into(),
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

        // Sign in as root
        db.signin(Root {
            username: &env.database_user,
            password: &env.database_password,
        })
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
        //
        // This can be extended as needed as the ontology and related
        // types are defined.
        db.query(
            "
DEFINE TABLE IF NOT EXISTS wollaaston SCHEMALESS;
DEFINE FIELD created ON TABLE wollaston TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD updated ON TABLE bill VALUE time::now();
",
        )
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        Ok(db)
    }
}
