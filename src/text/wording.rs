use std::fs;
use serde_json::{Result, Value};

 fn get_raw_data () -> String {
    let raw_data = fs::read_to_string("./assets/wording/eng.json")
        .expect("Something went wrong reading the file");
    return raw_data;
}

pub fn get() {
    let result = serde_json::from_str(&get_raw_data ());
   
    return ();
}