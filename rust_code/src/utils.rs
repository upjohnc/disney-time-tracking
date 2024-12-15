use crate::serial;
use chrono::prelude::Utc;
use serde_json::Result;
use std::fs::{read_to_string, File};
use std::io::Result as BaseResult;

#[allow(dead_code)]
pub fn read_json() -> Result<serial::BaseData> {
    let data = read_to_string("./data.json").expect("file bad");

    let mut p: serial::BaseData = serde_json::from_str(&data)?;

    let new_entry = serial::Entry("wham".to_string(), Utc::now().to_string());
    let new_date_entry = serial::DateEntry(vec![new_entry]);
    // p.0.insert("three".to_string(), new_date_entry);

    println!("{:?}", p);
    Ok(p)
}

#[allow(dead_code)]
pub fn write_json(the_data: &serial::BaseData) -> BaseResult<()> {
    let file = File::create("./updated_data.json")?;
    serde_json::to_writer_pretty(file, the_data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serial::{BaseData, Entry, RealEntry};
    use chrono::DateTime;
    use std::fs::read_to_string;

    #[test]
    fn test_ser_deser() {
        let data = read_to_string("./data.json").expect("file bad");
        let p: serial::BaseData = serde_json::from_str(&data).unwrap();
        let p_2: serial::BaseData = serde_json::from_str(&data).unwrap();
        let result_1 = serial::some_serialize(p);
        let result_2 = serial::some_deserialize(result_1);
        assert_eq!(p_2, result_2);
    }

    #[test]
    fn test_get_data() {
        let data = read_to_string("./data.json").expect("file bad");
        let base_data: serial::BaseData = serde_json::from_str(&data).unwrap();
        let binding = base_data.core_data();
        let entry_one = &binding.get("one").unwrap().0[0];

        assert_eq!(entry_one.0, "wow".to_string());
    }

    #[test]
    fn test_entry() {
        let entry = serial::Entry::new(
            "wow".to_string(),
            "2024-11-28 00:43:58.512246 UTC".to_string(),
        );
        let expected = RealEntry::new(
            "wow".to_string(),
            "2024-11-28 00:43:58.512246 UTC"
                .parse::<DateTime<Utc>>()
                .unwrap(),
        );
        let result = entry.go();
        assert_eq!(result, expected);
    }
}
