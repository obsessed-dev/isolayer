// create a get_voltage() function - f32
// read from a tmp file which stores echo'd voltage data
// implement fallback value so engine doesnt crash if echo doesnt work
// return it so the decision engine can process it

use crate::constants::VOLT_FILE;
use std::error::Error;
use tokio::fs;

pub async fn get_voltage() -> Result<f32, Box<dyn Error>> {
    let content = fs::read_to_string(VOLT_FILE).await?;
    let volt = content.trim().parse::<f32>()?;
    Ok(volt)
}
