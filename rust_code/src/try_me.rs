use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MySerialize(HashMap<String, String>);

impl MySerialize {
    // #[allow(dead_code)]
    // pub fn core_data(self) -> HashMap<String, String> {
    //     self.0
    // }
    #[allow(dead_code)]
    pub fn go(self) -> JustDigits {
        let mut new_hashmap = HashMap::new();
        for (key, value) in self.0.iter() {
            new_hashmap.insert(key.to_owned(), value.parse::<i32>().unwrap());
        }
        JustDigits(new_hashmap)
    }
}

#[derive(PartialEq, Debug)]
struct JustDigits(HashMap<String, i32>);

fn read_json() -> Result<MySerialize> {
    let data = read_to_string("./try_data.json").expect("openfile");

    let base: MySerialize = serde_json::from_str(&data)?;
    Ok(base)
}

fn full_thing() -> JustDigits {
    let data = read_json().expect("read file");
    let converted_data = data.go();
    converted_data
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_full_thing() {
        let result = full_thing();

        let serialize_data = HashMap::from([("one".to_string(), 12), ("two".to_string(), 13)]);
        let expected = JustDigits(serialize_data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read() {
        let result = read_json().unwrap();
        let serialize_data = HashMap::from([
            ("one".to_string(), "12".to_string()),
            ("two".to_string(), "13".to_string()),
        ]);
        assert_eq!(result, MySerialize(serialize_data));
    }

    #[test]
    fn test_serialize() {
        let the_hash = HashMap::from([("me".to_string(), "1".to_string())]);
        let input = MySerialize(the_hash);
        let the_hash = HashMap::from([("me".to_string(), 1)]);
        let expected = JustDigits(the_hash);
        let result = input.go();
        assert_eq!(expected, result);
    }
}
