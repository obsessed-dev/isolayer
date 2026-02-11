use crate::decision_engine::State;

#[derive(Debug)]
pub enum IsolayerEvent {
    TempUpdate(f32),
    VoltUpdate(f32),
    StateTransition { state: State, temp: f32, volt: f32 },
}
