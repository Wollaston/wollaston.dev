use axum::body::Body;
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use axum::Router;
use leptos::provide_context;
use leptos::*;
use leptos_axum::handle_server_fns_with_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use wollaston_dev::app::App;
use wollaston_dev::fileserv::file_and_error_handler;
use wollaston_dev::state::AppState;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), leptos::ServerFnError> {
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let pool = wollaston_dev::db::db().await?;

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            axum::routing::get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, axum::routing::get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<Body>) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.pool.clone());
        },
        App,
    );
    handler(req).await.into_response()
}
