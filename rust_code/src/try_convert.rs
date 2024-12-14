use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::utils::{BaseData, DateEntry, RealEntry};

#[derive(Debug)]
pub struct SerData(HashMap<String, RealDate>);

impl SerData {
    pub fn core_data(self) -> HashMap<String, RealDate> {
        self.0
    }

    pub fn new(a: HashMap<String, RealDate>) -> Self {
        Self(a)
    }
}

#[derive(Debug)]
pub struct RealDate(Vec<RealEntry>);

impl RealDate {
    pub fn core_data(self) -> Vec<RealEntry> {
        self.0
    }
}

impl RealDate {
    pub fn new(a: Vec<RealEntry>) -> Self {
        Self(a)
    }
}

// struct RealEntry(String, DateTime<Utc>);

pub fn some_serialize(input: BaseData) -> SerData {
    // pub fn some_serialize(input: BaseData) -> () {
    let mut ser_data = HashMap::new();
    let source_data = input.core_data();
    for (k, v) in source_data.into_iter() {
        let mut real_date = vec![];
        for e in v.0 {
            real_date.push(e.go());
        }
        ser_data.insert(k, RealDate::new(real_date));
    }
    SerData::new(ser_data)
}

pub fn some_deserialize(input: SerData) -> BaseData {
    let mut base_data = HashMap::new();
    let source_data = input.core_data();
    for (k, v) in source_data.into_iter() {
        let mut date_data = vec![];
        for e in v.core_data() {
            date_data.push(e.go());
        }
        base_data.insert(k, DateEntry::new(date_data));
    }
    BaseData::new(base_data)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::utils::Entry;

    use super::*;

    #[test]
    fn test_get_data() {
        let data = read_to_string("./data.json").expect("file bad");
        let base_data: BaseData = serde_json::from_str(&data).unwrap();
        // let entry_one = &base_data.core_data().get("one").unwrap().0[0];
        let binding = base_data.core_data();
        let entry_one = &binding.get("one").unwrap().0[0];

        assert_eq!(entry_one.0, "wow".to_string());
    }

    #[test]
    fn test_entry() {
        let entry = Entry::new(
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
