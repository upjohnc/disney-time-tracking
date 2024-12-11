use chrono::prelude::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Result as BaseResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseData(HashMap<String, DateEntry>);

impl BaseData {
    pub fn core_data(self) -> HashMap<String, DateEntry> {
        self.0
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DateEntry(Vec<Entry>);

#[derive(Deserialize, Serialize, Debug)]
struct Entry(String, String);

#[allow(dead_code)]
pub fn read_json() -> Result<BaseData> {
    let data = read_to_string("./data.json").expect("file bad");

    let mut p: BaseData = serde_json::from_str(&data)?;

    let new_entry = Entry("wham".to_string(), Utc::now().to_string());
    let new_date_entry = DateEntry(vec![new_entry]);
    p.0.insert("three".to_string(), new_date_entry);

    println!("{:?}", p);
    Ok(p)
}

#[allow(dead_code)]
pub fn write_json(the_data: &BaseData) -> BaseResult<()> {
    let file = File::create("./updated_data.json")?;
    serde_json::to_writer_pretty(file, the_data)?;
    Ok(())
}

