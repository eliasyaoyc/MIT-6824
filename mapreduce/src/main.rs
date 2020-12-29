use std::env;
use mapreduce::{sequential, map, reduce};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        usage()
    }

    let (args, files) = args.split_at(2);
    let files = files.to_owned();
    let jobname = args.get(1).unwrap().to_owned();

    sequential(
        jobname,
        files,
        3,
        map,
        reduce);
}

fn usage() {
    let usage = r"
    Can be run in 3 ways:
    1) Local (e.g. cargo run -- your_job_name ./data/test.txt)
    2) Master (e.g., cargo run --master 127.0.0.1:7777 x1.txt .. xN.txt)
    3) Worker (e.g., cargo run --worker 127.0.0.1:7777 127.0.0.1:8888 &)
    ";
    println!("{}",usage);
}