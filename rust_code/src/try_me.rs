use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
