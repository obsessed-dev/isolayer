// The decision engine is the heart of the "defined by data" mission.
// We use enums to represent these states because they are typesafe --
// It's impossible for the system to be in a state that doesn't exist.

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Active,  // Heater is ON
    Standby, // Heater is OFF
    Safety,  // Power level is not sufficient for ON-ness
}

pub struct DecisionEngine {
    current_state: State,
}

impl DecisionEngine {
    pub fn new() -> Self {
        Self {
            current_state: State::Standby,
        }
    }

    pub fn evaluation_policy(&mut self, temp: f32, volt: f32) -> State {
        // If voltage is too low, go to SAFETY regardless of temp
        if volt < 12.5 {
            self.current_state = State::Safety;
            return self.current_state;
        }

        match self.current_state {
            State::Active => {
                if temp >= 38.0 {
                    self.current_state = State::Standby;
                    return self.current_state;
                }
            }
            State::Standby | State::Safety => {
                if temp < 35.0 {
                    self.current_state = State::Active;
                    return self.current_state;
                }
            }
        }
        self.current_state
    }
}
