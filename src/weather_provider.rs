// get ambient temp function - f32
// read it from a tmp file which stores echo'd temp data
// implement fallback value so engine doesnt crash if data simulator is down
// return it so the decision engine can process it

use crate::constants::TEMP_FILE;
use crate::messages::IsolayerEvent;
use std::error::Error;
use std::path::Path;
use anyhow::Context;
use notify::{Config, RecommendedWatcher, Watcher};
use tokio::fs;
use tokio::sync::mpsc;

pub async fn run(tx: mpsc::Sender<IsolayerEvent>) {

    if let Ok(temp) = get_ambient_temp() {
        let _ = tx.send(IsolayerEvent::TempUpdate(temp)).await;
    }

    let (sync_tx, sync_rx) = mpsc::channel(32);
    let mut watcher = RecommendedWatcher::new(sync_tx, Config::default())
        .context("Failed to initialize watcher")?;

    let parent_dir = Path::new(TEMP_FILE).parent()
}

pub async fn get_ambient_temp() -> Result<f32, Box<dyn Error>> {
    let content = fs::read_to_string(TEMP_FILE).await?;
    let temp = content.trim().parse::<f32>()?;
    Ok(temp)
}
