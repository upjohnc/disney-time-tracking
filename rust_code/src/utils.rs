use chrono::prelude::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Result as BaseResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseData(HashMap<String, DateEntry>);

impl BaseData {
    pub fn new(a: HashMap<String, DateEntry>) -> Self {
        Self(a)
    }
    pub fn core_data(self) -> HashMap<String, DateEntry> {
        self.0
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DateEntry(pub Vec<Entry>);

impl DateEntry {
    pub fn new(a: Vec<Entry>) -> Self {
        Self(a)
    }

    pub fn core_data(self) -> Vec<Entry> {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct RealEntry(String, DateTime<Utc>);

impl RealEntry {
    pub fn new(a: String, b: DateTime<Utc>) -> Self {
        Self(a, b)
    }

    pub fn go(self) -> Entry {
        Entry(self.0, self.1.to_string())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry(pub String, pub String);

impl Entry {
    pub fn new(a: String, b: String) -> Self {
        Self(a, b)
    }

    pub fn go(self) -> RealEntry {
        RealEntry(self.0, self.1.parse::<DateTime<Utc>>().unwrap())
    }
}

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
