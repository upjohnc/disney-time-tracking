use crate::serial::{SerData, SerEntry};
use chrono::prelude::Utc;
use chrono::TimeDelta;
use serde_json::Result;
use std::collections::HashMap;
use std::env::var;
use std::fs::{read_to_string, File};
use std::io::Result as BaseResult;

const TAB_SPACE: &str = "  ";

fn get_file_location() -> String {
    format!(
        "{}/projects/personal/disney-time-tracking/rust_code/data.json",
        var("HOME").unwrap()
    )
}

pub fn retrieve_json() -> Option<SerData> {
    read_json(get_file_location()).expect("no errors")
}

pub fn read_json(file_path: String) -> Result<Option<SerData>> {
    let data = read_to_string(file_path).expect("file bad");

    let return_value = match data.chars().count() {
        0 => None,
        _ => {
            let p: SerData = serde_json::from_str(&data)?;
            Some(p)
        }
    };

    Ok(return_value)
}

pub fn write_json(the_data: &SerData) -> BaseResult<()> {
    let file = File::create(get_file_location())?;
    serde_json::to_writer_pretty(file, the_data)?;
    Ok(())
}

pub fn add_new_entry(entry_type: String, ser_data: Option<SerData>) -> SerData {
    let now = Utc::now();
    let date = now.date_naive().to_string();

    let entry = SerEntry::new(entry_type, now);

    let new_ser = match ser_data {
        Some(p) => {
            let mut core = p.core_data();

            match core.get_mut(&date) {
                Some(x) => {
                    x.push(entry);
                    ()
                }
                _ => {
                    core.insert(date, vec![entry]);
                    ()
                }
            }
            SerData::new(core)
        }
        None => {
            let mut h = HashMap::new();
            h.insert(date, vec![entry]);
            SerData::new(h)
        }
    };
    new_ser
}

pub fn new_entry(entry_type: String) {
    let ser = retrieve_json();
    let new_stuff = add_new_entry(entry_type, ser);
    let _ = write_json(&new_stuff);
}

pub fn calculate_time(ser: SerData) -> HashMap<String, TimeDelta> {
    let root_data = ser.core_data();
    let mut keys: Vec<String> = root_data.clone().into_keys().collect();
    keys.sort_unstable();

    let mut result = HashMap::new();

    for k in keys {
        let v = root_data.get(&k).unwrap();
        let mut day_time = vec![];
        // print the date
        println!("{}", k);
        for pair in v.chunks(2) {
            if pair.len() > 1 {
                let td = pair[1].give_date() - pair[0].give_date();
                day_time.push(td);

                // print the start-stop pair
                println!(
                    "{}{}{}:{}",
                    TAB_SPACE,
                    TAB_SPACE,
                    td.num_hours(),
                    td.num_minutes() % 60
                )
            }
        }
        let sum_day_time: TimeDelta = day_time.iter().sum();
        // print days total
        println!("{}Days Total", TAB_SPACE);
        println!(
            "{}{}{}:{}",
            TAB_SPACE,
            TAB_SPACE,
            sum_day_time.num_hours(),
            sum_day_time.num_minutes() % 60
        );
        result.insert(k, sum_day_time);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let expected = HashMap::from([
            ("2024-12-19".to_string(), TimeDelta::new(1980, 0).unwrap()),
            ("2024-10-28".to_string(), TimeDelta::new(0, 0).unwrap()),
            ("2024-12-17".to_string(), TimeDelta::new(5640, 0).unwrap()),
        ]);
        let data = read_json("./test_data.json".to_string())
            .expect("File ok")
            .unwrap();
        let result = calculate_time(data);
        assert_eq!(expected, result);
    }
}
