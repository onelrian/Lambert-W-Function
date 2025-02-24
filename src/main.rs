use anyhow::{Context, Result};
use lambert_w_function::{lambert_w, Cli};
use log::info;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let args = Cli::new();
    info!("Computing Lambert W for x = {}", args.get());

    match lambert_w(args.get()) {
        Ok(result) => {
            println!("Lambert W({}) = {}", args.get(), result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e).context("Failed to compute Lambert W function")
        }
    }
}