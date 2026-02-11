use crate::constants::STATUS_FILE;
use crate::decision_engine::State;
use tokio::{fs, io};

pub async fn apply_state(state: State, temp: f32, volt: f32) {
    match state {
        State::Active => {
            println!(
                "[Actuator] State: ACTIVE | Reason: [{:.2}\u{00B0}F | {:.2}V]",
                temp, volt
            );
            let _ = update_status_file("ACTIVE").await;
        }
        State::Standby => {
            println!("[Actuator] State: STANDBY | Reason: Thermal envelope maintained");
            let _ = update_status_file("STANDBY").await;
        }
        State::Safety => {
            eprintln!(
                "[Actuator] State: SAFETY Reason: {:.2}V is below 12.5V threshold",
                volt
            );
            let _ = update_status_file("SAFETY").await;
        }
    }
}

async fn update_status_file(status: &str) -> io::Result<()> {
    fs::write(STATUS_FILE, status).await
}
