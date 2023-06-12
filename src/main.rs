mod indexer;
use std::{path::Path, process::ExitCode};

use clap::Parser;
use indexer::Indexer;

/// HTTP server which allow searchs on images ( via EXIF data )
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Start indexing from this root directory
    root: String,

    /// Number of times to greet
    #[arg(default_value_t = 8080)]
    port: i16,
}

// fn build_index<'a>() -> Result<Indexer<'a>, ()> {
//     let args = Args::parse();
//     println!("{}", args.root);
//     println!("{}", args.port);

//     let root = Path::new(&args.root);
//     Indexer::new(&root);
//     Ok(Indexer::new(&root))
// }

fn main() -> ExitCode {
    let args = Args::parse();
    let root = Path::new(&args.root);
    match Indexer::new(&root) {
        Ok(index) => {
            print!("indexing complete about to start server");
            println!("{:#?}", index);
        }
        Err(_) => return ExitCode::FAILURE,
    }

    ExitCode::SUCCESS
}
