mod config;
mod error;
mod ibus;
mod log;

use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "ibus-vie-engine", version, about = "Vietnamese input method engine for IBus")]
struct Cli {
    /// Run as IBus engine (called by ibus-daemon).
    #[arg(long)]
    ibus: bool,

    /// Print version and exit.
    #[arg(long)]
    version: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("ibus-vie-engine {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    log::init();

    info!("ibus-vie-engine starting (version {})", env!("CARGO_PKG_VERSION"));

    let config = config::Config::load();
    info!(?config, "loaded configuration");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    rt.block_on(async {
        if let Err(e) = ibus::run(config).await {
            tracing::error!("engine error: {}", e);
            std::process::exit(1);
        }
    });
}
