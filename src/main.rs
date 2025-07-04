#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]

//! A web app to search a set of images.
//!
use std::path::PathBuf;

use clap::command;
use clap::Parser;

mod app;
mod component;
mod indexer;
mod pages;
mod util;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to index.
    root_dir: Option<PathBuf>,
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::io::Error;
    use std::io::ErrorKind;

    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;

    use crate::app::App;
    use crate::indexer::Index;
    use crate::pages::GLOBAL_STATE;

    let root_dir = match Args::parse().root_dir {
        Some(root_dir) => {
            if root_dir.is_dir() {
                root_dir
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Must supply a valid directory.",
                ));
            }
        }
        None => match std::env::current_dir() {
            Ok(root_dir) => root_dir,
            Err(_) => {
                tracing::error!(
                    "Could not read the current working directory."
                );
                return Err(Error::new(ErrorKind::Other, "No root directory supplied and could not read the current directory"));
            }
        },
    };

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.index = Index::new(root_dir.clone(), root_dir.clone());
            state
                .container_dir_set(root_dir.clone())
                .expect("Could not initialize container dir");
            state
                .list_dir_set(root_dir.clone())
                .expect("Could not initialize list dir");
            state
                .selected_dir_set(root_dir.clone())
                .expect("Could not initialize selected dir");
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::Other,"INTERNAL: Could not update global state from command line args"));
        }
    }

    let conf = match get_configuration(None) {
        Ok(conf) => conf,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::Other,
                "INTERNAL: Could not load configuration.",
            ));
        }
    };
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);
        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/images", &root_dir))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", format!("{site_root}")))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                      <!DOCTYPE html>
                      <html lang="en" class="dark:bg-slate-950 dark:text-white font-roboto">
                        <head>
                          <meta charset="utf-8" />
                          <meta name="description" content="Search images metadata." />
                          <meta name="viewport" content="width=device-width, initial-scale=1" />
                          <meta name="theme-color" content="#319197" />
                          <AutoReload options=leptos_options.clone() />
                          <HydrationScripts options=leptos_options.clone() />
                          <MetaTags />
                        </head>
                        <body>
                          <App />
                        </body>
                      </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

/// no client-side main function
/// unless we want this to work with e.g., Trunk for pure client-side testing
/// see lib.rs for hydration function instead
/// see optional feature `csr` instead
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}

/// a client-side main function is required for using `trunk serve`
/// prefer using `cargo leptos serve` instead
/// to run: `trunk serve --open --features csr`
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use start_actix::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
