#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! A web app the search a set of images.

mod image_gallery;
mod indexer;
mod pages;
mod sidebar;

extern crate seroost_lib;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::App;
    use actix_web::HttpServer;
    use leptos::get_configuration;
    use leptos::view;
    use leptos_actix::generate_route_list;
    use leptos_actix::LeptosRoutes;
    use photo_indexer::app::App;
    use tracing::log;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App/> });

    match HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! {  <App/> },
            )
            // TODO can I filter by extension rather than expose
            // all files from this directory.
            .service(Files::new("/exif-samples/", "../exif-samples/"))
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
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

#[cfg(not(any(feature = "ssr", feature = "csr")))]
/// Entry point not sure if this is reasonable yet....
pub fn main() {
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
    use leptos::*;
    use photo_indexer::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
