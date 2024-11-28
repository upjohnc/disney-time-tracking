use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::write;
use std::fs::{read_to_string, File};
use std::io::Result as BaseResult;

#[derive(Serialize, Deserialize, Debug)]
struct All {
    data: HashMap<String, DateEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct DateEntry(Vec<Entry>);

#[derive(Deserialize, Serialize, Debug)]
struct Entry(String, String);

#[allow(dead_code)]
fn read_json() -> Result<All> {
    let data = read_to_string("./data.json").expect("file bad");

    let mut p: All = serde_json::from_str(&data)?;

    let new_entry = Entry("wham".to_string(), Utc::now().to_string());
    let new_date_entry = DateEntry(vec![new_entry]);
    p.data.insert("three".to_string(), new_date_entry);

    println!("{:?}", p);
    Ok(p)
}

#[allow(dead_code)]
fn write_json(the_data: &All) -> BaseResult<()> {
    let file = File::create("./new_data.json")?;
    serde_json::to_writer_pretty(file, the_data)?;
    Ok(())
}

fn main() {
    let p = read_json().expect("should read");
    let _ = write_json(&p);

    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let _ = write("what.txt", utc.to_string());
    let x = "2024-11-28 00:40:27.648449 UTC".parse::<DateTime<Utc>>();
    println!("{:?}", x);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(1, 1);
    }
}
