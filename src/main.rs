mod doc_links;
mod gallery;
mod homepage;
mod indexer;

extern crate seroost_lib;

use seroost_lib::lexer::Lexer;
use seroost_lib::model::Model;

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
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    // simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    match HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
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
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `ssg` instead
}

// #[cfg(all(not(feature = "ssr"), feature = "csr"))]
// pub fn main() {
//     use leptos::*;
//     use leptos_start::app::*;
//     use wasm_bindgen::prelude::wasm_bindgen;
//     // a client-side main function is required for using `trunk serve`
//     // prefer using `cargo leptos serve` instead
//     // to run: `trunk serve --open --features ssg`

//     console_error_panic_hook::set_once();

//     leptos::mount_to_body(move |cx| {
//         // note: for testing it may be preferrable to replace this with a
//         // more specific component, although leptos_router should still work
//         view! {cx, <App/> }
//     });
// }
