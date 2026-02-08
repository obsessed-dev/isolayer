// The decision engine is the heart of the "defined by data" mission.
// We use enums to represent these states because they are typesafe --
// It's impossible for the system to be in a state that doesn't exist.

#[derive(Debug, PartialEq)]
pub enum SystemState {
    Active,     // Heater is ON
    Idle,       // Heater is OFF
    SafetyMode, // Power level is not sufficient for ON-ness
}

pub fn evaluation_policy(temp: f32, volt: f32) -> SystemState {
    if volt < 12.0 {
        return SystemState::SafetyMode;
    }

    if temp < 35.0 {
        return SystemState::Active;
    }

    return SystemState::Idle;
}
