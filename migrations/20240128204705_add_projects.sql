-- Add migration script here
INSERT INTO projects (title, path, github_link, description)
VALUES
  ("aratype", 
    "projects/aratype", 
    "https://github.com/Wollaston/aratype", 
    "A simple WASM app using Rust and Leptos that converts English letters to Arabic equivalents according to the Buckwalter transliteration table."
  ),
  ("wollaston.dev",
    "/",
    "https://github.com/Wollaston/wollaston.dev",
    "My personal website, built using Rust, Leptos, and axum."
  );

