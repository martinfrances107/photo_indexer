#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]

//! A web app the search a set of images.
mod component;
mod indexer;
mod pages;
mod util;

use clap::Parser;
use std::path::PathBuf;

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
    use actix_web::middleware::Compress;
    use actix_web::App;
    use actix_web::HttpServer;
    use leptos::hydration::AutoReload;
    use leptos::hydration::HydrationScripts;
    use leptos::prelude::get_configuration;
    use leptos::prelude::ClassAttribute;
    use leptos::prelude::ElementChild;
    use leptos::prelude::GlobalAttributes;
    use leptos::view;
    use leptos_actix::generate_route_list;
    use leptos_actix::LeptosRoutes;
    use leptos_meta::MetaTags;
    use leptos_meta::Stylesheet;
    use photo_indexer::app::App;
    use tracing::log;

    use crate::indexer::Index;
    use crate::pages::GLOBAL_STATE;
    use crate::pages::IMAGE_PREFIX;

    let args: Args = Args::parse();

    let root_dir = match args.root_dir {
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
                log::error!("Could not read the current working directory.");
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

    let config = match get_configuration(None) {
        Ok(conf) => conf,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::Other,
                "INTERNAL: Could not load configuration.",
            ));
        }
    };

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App /> });

    let addr = config.leptos_options.site_addr;
    match HttpServer::new(move || {
        let leptos_options = &config.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(Compress::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                // leptos_options.to_owned(),
                routes.to_owned(),
                {
                let value = config.leptos_options.clone();

                move || view! {
                  <!DOCTYPE html>
                  <html lang="en">
                    <head>
                      <meta name="description" content="Search images metadata." />
                      <meta name="viewport" content="width=device-width, initial-scale=1" />
                      <meta name="theme-color" content="#319197" />
                      <AutoReload options=value.clone() />
                      <HydrationScripts options=value.clone() />
                      <MetaTags />
                      <link rel="icon" type_="image/svg+xml" href="/mag.svg" />
                      <link rel="manifest" href="/manifest.json" />
                      <title text="Photo Indexer" />
                      <Stylesheet id="leptos" href="/pkg/pi.css" />
                    </head>
                    <body class="dark:bg-slate-950 dark:text-white font-roboto">
                      <App />
                    </body>
                  </html>
                }
              }
            )
            .service(favicon)
            .route("/manifest.json", actix_web::web::get().to(manifest))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/site", format!("{site_root}")))
            // TODO can I filter by extension rather than expose
            // all files from this directory.
            .service(Files::new(IMAGE_PREFIX, root_dir.clone()))
            // .service(Files::new("/", site_root))
    })
    .bind(&addr)
    {
        Ok(server) => {
            log::info!("Server started on  http://{addr}");
            server.run().await
        }
        Err(e) => {
            log::error!("Could not start server");
            Err(e)
        }
    }
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

#[cfg(feature = "ssr")]
async fn manifest(
    req: actix_web::HttpRequest,
) -> actix_web::Result<actix_files::NamedFile> {
    let path: PathBuf =
        req.match_info().query("manifest.json").parse().unwrap();
    let mut path: PathBuf = std::env::current_dir().unwrap();
    path.push("assets/manifest.json");
    Ok(actix_files::NamedFile::open(path)?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
/// Entry point not sure if this is reasonable yet....
pub const fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `ssg` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`

    use photo_indexer::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
