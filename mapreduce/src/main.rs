use std::env;
use mapreduce::{sequential, map, reduce, run_master, run_worker};

const MASTER_NODE: &str = "master";
const SEQUENTIAL: &str = "sequential";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        usage()
    } else if args[2] == *MASTER_NODE {
        let (args, files) = args.split_at(4);
        let files = files.to_owned();
        let job_name = args.get(1).unwrap().to_owned();
        let addr = args.get(3).unwrap().to_owned();

        if addr == *SEQUENTIAL {
            sequential(
                job_name,
                files,
                3,
                map,
                reduce);
        } else {
            run_master(
                job_name,
                files,
                3,
                addr,
            ).await.unwrap();
        }
    } else {
        run_worker(
            args[3].clone(),
            args[4].clone(),
        ).await;
    }
}

fn usage() {
    let usage = r"
    Can be run in 2 ways:
    1) Local (e.g. cargo run --your_job_name master sequential ./data/test.txt)
    2) Master (e.g., cargo run --your_job_name master 127.0.0.1:7777 ./data/test.txt)
       & Worker (e.g., cargo run --your_job_name worker 127.0.0.1:7777 127.0.0.1:8888)
    ";
    println!("{}", usage);
}