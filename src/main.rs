use chrono::Timelike;
use chrono::NaiveTime;
use serde:: {Serialize, Deserialize};
use std::fs;
mod engine;
use engine::{Block, Config, build_schedule};


fn main() {
    let json_config = fs::read_to_string("config.json").map_err(|e| format!("Unable to Read JSON"));
}

fn convert_time(time: &str) -> Result<u32, String> {

    let format = "%I:%M%P";
    let converted_time = NaiveTime::parse_from_str(time, format).map_err(|e| format!("Unable to Parse Time: {}", e))?;
    Ok(converted_time.hour() * 3600 + converted_time.minute() * 60 + converted_time.second())


}

fn json_to_struct(data: &str) -> Result<Block, String> {
    Ok(serde_json::from_str::<Block>(data).map_err(|e| format!("Unable to Convert JSON to Struct"))?)
}

fn json_to_config(data: &str) -> Result<Config, String> {
    Ok(serde_json::from_str::<Config>(data).map_err(|e| format!("Unable to Convert JSON to Config"))?)
}