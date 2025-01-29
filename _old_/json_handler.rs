use serde::{Deserialize, Serialize};
use std::fs;

pub fn read_json<T: for<'de> Deserialize<'de>>(file_path: &str) -> T {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Error parsing JSON")
}

pub fn write_json<T: Serialize>(file_path: &str, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("Error serializing JSON");
    fs::write(file_path, json).expect("Unable to write file");
}