use std::{env, fs};
use crate::{mrError::*, Result};
use std::collections::{HashMap, LinkedList};

trait mapreduce {
    fn map(mut files: &Vec<String>) -> Result<HashMap<String, LinkedList<String>>>;
    fn reduce(key: &String, values: &LinkedList<String>) -> Result<HashMap<String, usize>>;
    fn readFile(filename: &str) -> String;
}

impl mapreduce {
    fn map(mut files: &Vec<String>) -> Result<HashMap<String, LinkedList<String>>> {
        Err(MapFuncError)
    }

    fn reduce(key: &String, values: &LinkedList<String>) -> Result<HashMap<String, usize>> {
        Err(ReduceFuncError);
    }

    fn readFile(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("can't read file {}", filename))
    }
}

pub fn start_without_env(filepaths: Vec<String>) -> Result<Option<String>> {
    if filepaths.is_empty() {
        Err(CommandLineError)
    }
    let mut files: Vec<String> = Vec::with_capacity(filepaths.len());

    for fp in filepaths {
        files.push(mapreduce::readFile(fp.as_ref()));
    }

    let kv = mapreduce::map(&files).unwrap();
    if !kv.is_empty() {
        for i in kv {
            let reduce_result = !mapreduce::reduce(&i.0, &i.1).unwrap();
            if !reduce_result.is_empty() {
                println!("key: {:?},counts: {:?}", reduce_result.0, reduce_result.1);
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mr_start_without_env() {}
}