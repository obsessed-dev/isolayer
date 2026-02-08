mod actuator;
mod constants;
mod decision_engine;
mod power_regulator;
mod weather_provider;

use tokio::main;
use tokio::time::{interval, Duration};

#[main]
async fn main() {
    println!("[Isolayer] Starting up...");

    let mut heartbeat = interval(Duration::from_secs(60));

    loop {
        println!("[Isolayer] Watching for events...");
        heartbeat.tick().await;

        let temp_reading = weather_provider::get_ambient_temp().await;
        let volt_reading = power_regulator::get_voltage().await;

        if let (Ok(t), Ok(v)) = (temp_reading, volt_reading) {
            println!("Data aquired: {:.2}\u{00B0}F | {:.2}V", t, v);

            let state = decision_engine::evaluation_policy(t, v);
            actuator::apply_state(state, t, v).await;
        } else {
            eprintln!("Warning: Failed to aquire full data stream. Retrying...");
        }
    }
}
