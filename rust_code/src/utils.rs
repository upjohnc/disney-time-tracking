use crate::serial;
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
        "{}/projects/disney-time-tracking/rust_code/data.json",
        var("HOME").unwrap()
    )
}

pub fn read_json() -> Result<Option<serial::StringRootData>> {
    let data = read_to_string(get_file_location()).expect("file bad");

    let return_value = match data.chars().count() {
        0 => None,
        _ => {
            let p: serial::StringRootData = serde_json::from_str(&data)?;
            Some(p)
        }
    };

    Ok(return_value)
}

pub fn write_json(the_data: &serial::StringRootData) -> BaseResult<()> {
    let file = File::create(get_file_location())?;
    serde_json::to_writer_pretty(file, the_data)?;
    Ok(())
}

pub fn add_new_entry(entry_type: String, ser_data: Option<serial::SerData>) -> serial::SerData {
    let now = Utc::now();
    let date = now.date_naive().to_string();

    let entry = serial::SerEntry::new(entry_type, now);

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
            serial::SerData::new(core)
        }
        None => {
            let mut h = HashMap::new();
            h.insert(date, vec![entry]);
            serial::SerData::new(h)
        }
    };
    new_ser
}

pub fn new_entry(entry_type: String) {
    let ser = serial::retrieve_json();
    let new_stuff = add_new_entry(entry_type, ser);
    let out = serial::do_deserialize(new_stuff);
    let _ = write_json(&out);
}

pub fn calculate_time() {
    let ser = serial::retrieve_json();

    let root_data = ser.unwrap().core_data();
    let mut keys: Vec<String> = root_data.clone().into_keys().collect();
    keys.sort_unstable();

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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serial::SerEntry;
    use chrono::DateTime;
    use std::fs::read_to_string;

    #[test]
    fn test_ser_deser() {
        let data = read_to_string("./data.json").expect("file bad");
        let p: serial::StringRootData = serde_json::from_str(&data).unwrap();
        let p_2: serial::StringRootData = serde_json::from_str(&data).unwrap();
        let result_1 = serial::do_serialize(p);
        let result_2 = serial::do_deserialize(result_1);
        assert_eq!(p_2, result_2);
    }

    #[test]
    fn test_get_data() {
        let data = read_to_string("./test_data.json").expect("file bad");
        let base_data: serial::StringRootData = serde_json::from_str(&data).unwrap();
        let binding = base_data.core_data();
        let entry_one = &binding.get("2024-12-16").unwrap()[0];

        assert_eq!(entry_one.first_element(), "wow".to_string());
    }

    #[test]
    fn test_entry() {
        let entry = serial::StringEntry::new(
            "wow".to_string(),
            "2024-11-28 00:43:58.512246 UTC".to_string(),
        );
        let expected = SerEntry::new(
            "wow".to_string(),
            "2024-11-28 00:43:58.512246 UTC"
                .parse::<DateTime<Utc>>()
                .unwrap(),
        );
        let result = entry.convert_to_datetime();
        assert_eq!(result, expected);
    }
}
