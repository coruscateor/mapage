use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Command
{

    id: Option<u32>,
    command: String,
    for_type: Option<String>,
    //namespace: Option<String>,
    bool_value: Option<bool>,
    f32_value: Option<f32>
    
}