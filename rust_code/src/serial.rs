use chrono::prelude::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SerData(HashMap<String, RealDate>);

impl SerData {
    pub fn core_data(self) -> HashMap<String, RealDate> {
        self.0
    }

    pub fn new(a: HashMap<String, RealDate>) -> Self {
        Self(a)
    }
}

#[derive(Debug, PartialEq)]
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

pub fn some_serialize(input: BaseData) -> SerData {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BaseData(HashMap<String, DateEntry>);

impl BaseData {
    pub fn new(a: HashMap<String, DateEntry>) -> Self {
        Self(a)
    }
    pub fn core_data(self) -> HashMap<String, DateEntry> {
        self.0
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DateEntry(pub Vec<Entry>);

impl DateEntry {
    pub fn new(a: Vec<Entry>) -> Self {
        Self(a)
    }

    pub fn core_data(self) -> Vec<Entry> {
        self.0
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Entry(pub String, pub String);

impl Entry {
    pub fn new(a: String, b: String) -> Self {
        Self(a, b)
    }

    pub fn go(self) -> RealEntry {
        RealEntry(self.0, self.1.parse::<DateTime<Utc>>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_ser_deser() {
        let data = read_to_string("./data.json").expect("file bad");
        let p: BaseData = serde_json::from_str(&data).unwrap();
        let p_2: BaseData = serde_json::from_str(&data).unwrap();
        let result_1 = some_serialize(p);
        let result_2 = some_deserialize(result_1);
        assert_eq!(p_2, result_2);
    }

    #[test]
    fn test_get_data() {
        let data = read_to_string("./data.json").expect("file bad");
        let base_data: BaseData = serde_json::from_str(&data).unwrap();
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
