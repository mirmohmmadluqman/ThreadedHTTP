use clap::Parser;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use threaded_http::{start_server, ServerConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "7878")]
    port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short = 't', long, default_value = "4")]
    threads: usize,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let config = ServerConfig {
        address: format!("{}:{}", args.host, args.port),
        pool_size: args.threads,
        verbose: args.verbose,
    };

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    ctrlc::set_handler(move || {
        println!("\nReceived shutdown signal...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })
    .unwrap_or_else(|err| {
        eprintln!("Error setting Ctrl-C handler: {}", err);
    });

    println!("Starting ThreadedHTTP server on {}", config.address);
    println!("Thread pool size: {}", config.pool_size);
    println!("Press Ctrl+C to stop\n");

    if let Err(e) = start_server(config, shutdown) {
        eprintln!("Server error: {}", e);
        process::exit(1);
    }
}