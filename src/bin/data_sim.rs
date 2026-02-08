use anyhow::{Context, Ok, Result};
use rand::Rng;
use std::fs;
use std::process;
use std::thread;
use std::time::Duration;
mod constants;

use crate::constants::{TEMP_FILE, TMP_DIR, VOLT_FILE};

fn main() -> Result<()> {
    let mut rng = rand::rng();

    let path = TMP_DIR;

    fs::create_dir_all(&path).context("Failed to initialize the mock data directory in /tmp")?;

    // Catch OS interrupt signal on program finish
    ctrlc::set_handler(move || {
        println!("\n[Isolayer Data Simulator]: Shutting down. Cleaning up /tmp directory...");
        let _ = fs::remove_dir_all(&path);
        process::exit(0);
    })
    .context("Failed to remove the mock data directory in /tmp")?;

    println!("[Isolayer Data Simulator]: Generating randomized environmental data...");

    loop {
        // generating random but realistic float values
        let temp = rng.random_range(-5.0..45.0);
        let volt = rng.random_range(11.5..14.0);

        // writing to our 'mock registers' (files)
        fs::write(TEMP_FILE, temp.to_string()).context("Failed to write data to temp file")?;
        fs::write(VOLT_FILE, volt.to_string()).context("Failed to write data to volt file")?;

        println!("Updated: {:.2}\u{00B0}F | {:.2}V", temp, volt);

        // The daemon will check every 60 seconds, but we'll post every 15 seconds to test this robustly.
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
