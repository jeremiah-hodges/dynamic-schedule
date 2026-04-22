use chrono::NaiveTime;
use serde:: {Serialize, Deserialize};


fn main() {
    println!("Hello, world!");
}

fn convert_time(time: &str) -> Result<u32, String> {

    let format = "%I:%M%P";
    let converted_time = NaiveTime::parse_from_str(time, format).map_err(|e| format!("Unable to Parse Time: {}", e))?;
    Ok(converted_time.num_seconds_from_midnight())

}


#[derive(Serialize, Deserialize, Debug)]
struct Block{
    name: String,
    start: u32,
    end: u32,
    activity: String,
    tag: String, 
    note: String,
}

fn json_to_struct(data: &str) -> Result<Block, String> {
   Ok(serde_json::from_str(data).map_err(|e| format!("Unable to Convert JSON to struct")))?
}
