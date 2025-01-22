use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

pub mod components;
pub mod routes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/wollaston-dev.css"/>

        <Title text="wollaston.dev"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("/") view=components::Layout >
                        <Route path=path!("") view=routes::home::HomePage/>
                        <Route path=path!("/blog" ) view=routes::blog::Blog/>
                        <Route path=path!("/blog/:slug") view=routes::blog::slug::Slug/>
                        <Route path=path!("/projects") view=routes::projects::Project/>
                        <Route path=path!("/projects/aratype") view=routes::projects::aratype::Aratype/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
