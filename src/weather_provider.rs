// get ambient temp function - f32
// read it from a tmp file which stores echo'd temp data
// implement fallback value so engine doesnt crash if data simulator is down
// return it so the decision engine can process it

use crate::constants::TEMP_FILE;
use std::error::Error;
use tokio::fs;

pub async fn get_ambient_temp() -> Result<f32, Box<dyn Error>> {
    let content = fs::read_to_string(TEMP_FILE).await?;
    let temp = content.trim().parse::<f32>()?;
    Ok(temp)
}
