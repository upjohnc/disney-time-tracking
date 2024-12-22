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

#[allow(dead_code)]
fn old_code() {
    let p = read_json().expect("should read");
    let _ = write_json(&p);

    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let _ = write("datetime.txt", utc.to_string());
    let x = "2024-11-28 00:40:27.648449 UTC".parse::<DateTime<Utc>>();
    println!("{:?}", x);
}

#[derive(Deserialize, Serialize, Debug)]
struct VecData(Vec<String>);

#[allow(dead_code)]
fn read_array_struct() -> Result<VecData> {
    let data = read_to_string("datetime_array.json").expect("file bad");
    let v: VecData = serde_json::from_str(&data)?;
    Ok(v)
}

#[allow(dead_code)]
fn read_array() -> Result<Value> {
    let data = read_to_string("datetime_array.json").expect("file bad");
    let v: Value = serde_json::from_str(&data)?;
    Ok(v)
}

#[allow(dead_code)]
fn convert_array_datetime_struct(data: VecData) -> () {
    for i in data.0 {
        println!("{}", i);
    }
    ()
}

#[allow(dead_code)]
fn convert_array_datetime(data: Value) -> Vec<DateTime<Utc>> {
    let mut new_array = vec![];
    if let Some(v) = data.as_array() {
        for i in v {
            let my_date = i.as_str().unwrap().parse::<DateTime<Utc>>();
            new_array.push(my_date.unwrap());
            println!("{:?}", my_date);
        }
    }
    new_array
}

#[allow(dead_code)]
fn loop_over(d: BaseData) {
    for (i, j) in d.core_data() {
        println!(" zoom {}, {:?}", i, j);
    }
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
