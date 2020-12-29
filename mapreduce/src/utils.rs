use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum MrPhase {
    MapPhase,
    ReducePhase,
}

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

pub fn reduce_name(job_name: &str, map_task: usize, reduce_task: usize) -> String {
    format!("mrtmp.{}-{}-{}", job_name, map_task, reduce_task)
}

pub fn merge_name(job_name: &str, reduce_task: usize) -> String {
    format!("mrtmp.{}-res-{}", job_name, reduce_task)
}

pub fn hash_key(key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    hasher.finish() as usize
}

pub fn merge(job_name: &str, n_reduce:usize){
    let mut kvs: HashMap<String, String> = HashMap::new();
    for i in 0..n_reduce {
        let filename = merge_name(job_name, i);
        let mut file_path = PathBuf::from("./data");
        file_path.push(filename);

        let file = fs::File::open(file_path).expect("read file failed");
        let file = BufReader::new(file);
        for line in file.lines() {
            let line = line.unwrap();
            let kv: KeyValue = serde_json::from_str(&line).expect("deserialize failed");
            kvs.insert(kv.key.clone(), kv.value.clone());
        }
    }

    let mut keys: Vec<&str> = Vec::new();
    for k in kvs.keys() {
        keys.push(k);
    }

    keys.sort();

    let mut file = fs::File::create(&format!("./data/mrtmp.{}", job_name)).expect("create file failed");

    for k in keys {
        file.write_all(&format!("{}:{}\n", k, kvs.get(k).unwrap()).into_bytes()).expect("write failed");
    }
}