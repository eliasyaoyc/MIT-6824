use crate::{KeyValue, read_file, reduce_name, hash_key};
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;


pub fn common_map(
    job_name: &str,
    map_task: usize,
    file: &str,
    n_reduce: usize,
    map_func: fn(&str, &str) -> Vec<KeyValue>,
) {
    let contents = read_file(file);

    let mut reduce_files: Vec<fs::File> = Vec::with_capacity(n_reduce);

    let in_file_path = PathBuf::from(file);
    let in_file_dir = in_file_path.parent().unwrap();

    for i in 0..n_reduce {
        let filename = reduce_name(job_name, map_task, i);
        let mut out_file = PathBuf::from(&in_file_dir);
        out_file.push(filename);
        let file = fs::File::create(&out_file).expect(&format!("create file {:?} failed", out_file));
        reduce_files.push(file)
    }

    let kvs: Vec<KeyValue> = map_func(file, &contents);

    for kv in kvs {
        let r = hash_key(&kv.key) % n_reduce;
        let mut inter_file = &reduce_files[r];
        let json_string = serde_json::to_string(&kv).expect("failed to convert kv to string");

        inter_file.write_all(&json_string.into_bytes()).expect("write string to file failed");
        inter_file.write_all("\n".as_bytes()).expect("write \\n failed");
    }
}