use clap::Parser;
use ibus_vie_im::{Engine, TelexEngine, VniEngine, ViqrEngine};
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(name = "ibus-vie-cli", version, about = "Debug tool for ibus-vie FSM")]
struct Cli {
    /// Input method: telex, vni, or viqr.
    #[arg(long, default_value = "telex")]
    method: String,

    /// One-shot input: process this string and print result.
    #[arg(long)]
    input: Option<String>,

    /// Trace mode: print each FSM step.
    #[arg(long)]
    trace: bool,
}

fn create_engine(method: &str) -> Box<dyn Engine> {
    match method {
        "vni" => Box::new(VniEngine::new()),
        "viqr" => Box::new(ViqrEngine::new()),
        _ => Box::new(TelexEngine::new()),
    }
}

fn main() {
    let cli = Cli::parse();

    let mut engine = create_engine(&cli.method);

    // One-shot mode
    if let Some(input) = &cli.input {
        let result = engine.feed_str(input);
        println!("{}", result);
        return;
    }

    // Interactive mode
    println!(
        "ibus-vie-cli ({}). Gõ vào, Ctrl+D để thoát.",
        cli.method
    );

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let result = engine.feed_str(&line);
        writeln!(stdout, "{}", result).ok();
        stdout.flush().ok();
    }
}
