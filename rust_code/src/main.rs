mod serial;
mod utils;

use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serial::{some_deserialize, some_serialize, BaseData};
use std::fs::{read_to_string, write};
use utils::{read_json, write_json};

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

fn main() {
    // let array = read_array_struct();
    // let array = convert_array_datetime_struct(array.unwrap());
    // let array = read_array();
    // let array = convert_array_datetime(array.unwrap());

    // let data = read_json();
    // loop_over(data.unwrap());
    // println!("Hello, world!");
    let data = read_to_string("./data.json").expect("file bad");
    let p: BaseData = serde_json::from_str(&data).unwrap();
    let result_1 = some_serialize(p);
    println!("{:?}", result_1);
    let result_2 = some_deserialize(result_1);
    println!("{:?}", result_2);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(1, 1);
    }
}
