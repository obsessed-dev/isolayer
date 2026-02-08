use crate::constants::STATUS_FILE;
use crate::decision_engine::SystemState;
use tokio::{fs, io};

pub async fn apply_state(state: SystemState, temp: f32, volt: f32) {
    match state {
        SystemState::Active => {
            println!(
                "[Actuator] State: ACTIVE | Reason: {:.2}\u{00B0}F < 35\u{00B0}F",
                temp
            );
            let _ = update_status_file("ACTIVE").await;
        }
        SystemState::Idle => {
            println!("[Actuator] State: IDLE | Reason: Thermal envelope maintained");
            let _ = update_status_file("IDLE").await;
        }
        SystemState::SafetyMode => {
            eprintln!(
                "[Actuator] State: SAFETY Reason: {:.2}V is below 12.2V threshold",
                volt
            );
            let _ = update_status_file("SAFETY").await;
        }
    }
}

async fn update_status_file(status: &str) -> io::Result<()> {
    fs::write(STATUS_FILE, status).await
}
