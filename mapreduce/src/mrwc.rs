use crate::KeyValue;

#[allow(unused_variables)]
pub fn map(filename: &str, contents: &str) -> Vec<KeyValue> {
    let words: Vec<&str> = contents.split_whitespace().collect();

    let mut keyval: Vec<KeyValue> = Vec::new();
    for w in words {
        let kv_obj = KeyValue {
            key: w.to_string(),
            value: String::from("1"),
        };
        keyval.push(kv_obj);
    }

    keyval
}

#[allow(unused_variables)]
pub fn reduce(key: &str, value: &Vec<String>) -> String {
    value.len().to_string()
}