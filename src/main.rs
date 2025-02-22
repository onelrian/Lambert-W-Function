use anyhow::{Context, Result};
use lambert::lambert_w;
use log::info;
use input::Cli;




fn main() -> Result<(),anyhow::Error> {
    env_logger::init();
   
    let args = Cli::new();
    info!("Computing Lambert W for x = {}", Cli::get(&args));

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
mod lambert;
mod input;