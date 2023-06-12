use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("{}", args.root);
    println!("{}", args.port);
}
