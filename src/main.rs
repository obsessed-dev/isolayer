mod actuator;
mod constants;
mod decision_engine;
mod messages;
mod power_regulator;
mod weather_provider;

use crate::decision_engine::{DecisionEngine, State};
use crate::messages::IsolayerEvent;

use tokio::{main, spawn, sync::mpsc};

#[main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<IsolayerEvent>(32);

    let mut engine = DecisionEngine::new();
    let mut current_temp = 30.0;
    let mut current_volt = 13.0;

    spawn(weather_provider::run(tx.clone()));
    spawn(power_regulator::run(tx.clone()));

    println!("[Isolayer] Event Bus Active...");

    while let Some(event) = rx.recv().await {
        match event {
            IsolayerEvent::TempUpdate(t) => {
                current_temp = t;
                if let Some(transition) = engine.evaluation_policy(current_temp, current_volt) {
                    let _ = tx.send(transition).await;
                }
            }
            IsolayerEvent::VoltUpdate(v) => {
                current_volt = v;
                if let Some(transition) = engine.evaluation_policy(current_temp, current_volt) {
                    let _ = tx.send(transition).await;
                }
            }
            IsolayerEvent::StateTransition { state, temp, volt } => {
                actuator::apply_state(state, temp, volt).await;
            }
        }
    }
}
