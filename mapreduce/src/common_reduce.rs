use std::fs;
use crate::{reduce_name, KeyValue};
use std::env;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

pub fn common_reduce(
    job_name: &str,
    reduce_task: usize,
    outfile: &str,
    n_map: usize,
    reduce_func: fn(&str, &Vec<String>) -> String,
) {
    let mut inter_files: Vec<fs::File> = Vec::with_capacity(n_map);

    for i in 0..n_map {
        let filename = reduce_name(job_name, i, reduce_task);
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push("data");
        current_dir.push(filename);

        let file = fs::File::open(current_dir).expect("open file failed");
        inter_files.push(file);
    }

    let mut kvs: Vec<KeyValue> = Vec::new();
    for file in inter_files {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let kv: KeyValue = serde_json::from_str(&line.unwrap()).expect("parse string to kv failed");
            kvs.push(kv);
        }
    }

    kvs.sort_by(|l, r| l.key.partial_cmp(&r.key).unwrap());

    let mut out_file_path = PathBuf::from("./data");
    out_file_path.push(outfile);
    println!("================ : {:?}",out_file_path);
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(out_file_path)
        .expect("append or create file failed");

    let mut i = 0;
    while i < kvs.len() {
        let mut vals: Vec<String> = Vec::new();
        let mut j = i;
        while j < kvs.len() && kvs[i].key == kvs[j].key {
            vals.push(kvs[j].value.clone());
            j += 1;
        }

        let result_kv = KeyValue {
            key: kvs[i].key.clone(),
            value: reduce_func(&kvs[i].key, &vals),
        };

        file.write_all(&serde_json::to_string(&result_kv).unwrap().into_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write \\n failed");
        i = j;
    }
}
