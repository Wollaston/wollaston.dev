use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wollaston-dev.css"/>

        // sets the document title

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=components::Layout>
                        <Route path="" view=routes::home::HomePage/>
                        <Route path="/blog"  view=routes::blog::Blog/>
                        <Route path="/blog/:slug" view=routes::blog::slug::Slug/>
                        <Route path="/projects" view=routes::projects::Project/>
                        <Route path="/projects/aratype" view=routes::projects::aratype::Aratype/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
