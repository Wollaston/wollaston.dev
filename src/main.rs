use std::error::Error;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use axum::Router;
    use leptos::logging;
    use leptos::prelude::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes};
    use wollaston_dev::app::*;
    use wollaston_dev::config::AppState;

    let app_state = AppState::new().await?;

    let addr = app_state.leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let opts = app_state.clone().leptos_options;
                move || shell(opts.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
