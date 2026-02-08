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

    let mut engine = decision_engine::DecisionEngine::new();
    let mut last_state: Option<decision_engine::State> = None;

    let mut heartbeat = interval(Duration::from_secs(60));

    println!("[Isolayer] Watching for events...");

    loop {
        heartbeat.tick().await;

        let (temp_res, volt_res) = tokio::join!(
            weather_provider::get_ambient_temp(),
            power_regulator::get_voltage()
        );

        let current_temp = temp_res.unwrap_or_else(|_| {
            eprintln!("Weather API failed, using fail-safe temp...");
            30.0
        });

        match volt_res {
            Ok(v) => {
                let current_state = engine.evaluation_policy(current_temp, v);
                if Some(current_state) != last_state {
                    actuator::apply_state(current_state, current_temp, v).await;
                    last_state = Some(current_state);
                }
            }
            Err(e) => eprintln!("CRITICAL: Could not read voltage: {}. Skipping cycle...", e),
        }
    }
}
