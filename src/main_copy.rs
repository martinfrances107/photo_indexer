// mod counter;
// mod indexer;

// use std::{path::Path, process::ExitCode};

// use clap::Parser;
// use counter::SimpleCounter;
// use indexer::Indexer;
// use leptos::mount_to_body;
// use leptos::view;

// /// HTTP server which allow searchs on images ( via EXIF data )
// #[derive(Parser)]
// #[command(author, version, about)]
// struct Args {
//     /// Start indexing from this root directory
//     root: String,

//     /// Number of times to greet
//     #[arg(default_value_t = 8080)]
//     port: i16,
// }

// fn main() -> ExitCode {
//     let args = Args::parse();
//     let root = Path::new(&args.root);
//     match Indexer::new(root) {
//         Ok(index) => {
//             print!("indexing complete about to start server");
//             println!("{:#?}", index);
//         }
//         Err(_) => return ExitCode::FAILURE,
//     }
//     _ = console_log::init_with_level(log::Level::Debug);
//     console_error_panic_hook::set_once();
//     mount_to_body(|cx| {
//         view! { cx,
//             <SimpleCounter
//                 initial_value=0
//                 step=1
//             />
//         }
//     });
//     ExitCode::SUCCESS
// }
