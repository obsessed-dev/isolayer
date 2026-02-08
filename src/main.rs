mod actuator;
mod decision_engine;
mod power_regulator;
mod provider;

use std::time::Duration;
use tokio::{main, time};

#[main]
async fn main() {
    println!("Isolayer is starting up...");

    loop {
        println!("Isolayer is listening...");

        time::sleep(Duration::from_secs(15)).await;
    }
}
