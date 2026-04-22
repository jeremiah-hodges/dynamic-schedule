use serde::{Serialize, Deserialize};
pub fn build_schedule(config: &Config) -> Vec<Block>{
Vec::new()
}



#[derive(Deserialize, Debug)]
pub struct Config{
   wake_time: u64,
   gym_earliest: u64,
   gym_duration: u64,
   goal_work_hours: u32,
   sleep_anchor: u64,
   day_type: String, 
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Block{
    name: String,
    start: u32,
    end: u32,
    activity: String,
    tag: String, 
    note: String,
}
