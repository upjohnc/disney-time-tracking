use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
struct All {
    data: HashMap<String, DateEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct DateEntry(Vec<Entry>);

#[derive(Deserialize, Serialize, Debug)]
struct Entry(String, String);

#[allow(dead_code)]
fn read_json() -> Result<All> {
    let data = fs::read_to_string("./data.json").expect("file bad");

    let mut p: All = serde_json::from_str(&data)?;

    let new_entry = Entry("wham".to_string(), "zoom".to_string());
    let new_date_entry = DateEntry(vec![new_entry]);
    p.data.insert("three".to_string(), new_date_entry);

    Ok(p)
}

#[allow(dead_code)]
fn write_json(the_data: All) -> Result<()> {
    let string_data = serde_json::to_string(&the_data)?;
    fs::write("./new_data.json", string_data).expect("file should save");
    Ok(())
}

fn main() {
    let p = read_json();
    let _ = write_json(p.expect("should save"));

    println!("Hello, world!");
}
