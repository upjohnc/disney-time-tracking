use crate::utils;
use chrono::prelude::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SerData(HashMap<String, Vec<SerEntry>>);

impl SerData {
    pub fn new(a: HashMap<String, Vec<SerEntry>>) -> Self {
        Self(a)
    }

    pub fn core_data(self) -> HashMap<String, Vec<SerEntry>> {
        self.0
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SerEntry(String, DateTime<Utc>);

impl SerEntry {
    pub fn new(a: String, d: DateTime<Utc>) -> Self {
        Self(a, d)
    }
    pub fn give_date(&self) -> DateTime<Utc> {
        self.1.clone()
    }
}

pub fn retrieve_json() -> Option<SerData> {
    utils::read_json().expect("no errors")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_serialize() {
        let data = r#"
            {
              "2024-12-22": [
                [
                  "start",
                  "2024-12-22 21:49:04.082835 UTC"
                ]
              ]
            }"#;

        let mut base_hash_map = HashMap::new();
        let zz = vec![SerEntry::new(
            "start".to_string(),
            "2024-12-22 21:49:04.082835 UTC"
                .parse::<DateTime<Utc>>()
                .unwrap(),
        )];
        base_hash_map.insert("2024-12-22".to_string(), zz);
        let expected = SerData::new(base_hash_map);
        let result: SerData = serde_json::from_str(data).unwrap();
        assert_eq!(result, expected)
    }
}
