// create a get_voltage() function - f32
// read from a tmp file which stores echo'd voltage data
// implement fallback value so engine doesnt crash if echo doesnt work
// return it so the decision engine can process it

use crate::constants::VOLT_FILE;
use crate::messages::IsolayerEvent;
use std::error::Error;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

pub async fn run(tx: mpsc::Sender<IsolayerEvent>) {
    let mut heartbeat = interval(Duration::from_secs(15));

    loop {
        heartbeat.tick().await;
        match get_voltage().await {
            Ok(volt) => {
                let _ = tx.send(IsolayerEvent::VoltUpdate(volt)).await;
            }
            Err(e) => eprintln!("[Power] Read error: {}", e),
        }
    }
}

pub async fn get_voltage() -> Result<f32, Box<dyn Error + Send + Sync + 'static>> {
    let content = fs::read_to_string(VOLT_FILE).await?;
    let volt = content.trim().parse::<f32>()?;
    Ok(volt)
}
