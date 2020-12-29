use std::thread;
use crate::{KeyValue, MrPhase, merge_name, merge};
use crate::common_map::common_map;
use crate::common_reduce::common_reduce;
use log::info;

#[allow(non_snake_case)]
pub fn sequential(
    job_name: String,
    files: Vec<String>,
    n_reduce: usize,
    mapFunc: fn(&str, &str) -> Vec<KeyValue>,
    reduceFunc: fn(&str, &Vec<String>) -> String,
) {
    let handle = thread::Builder::new()
        .name("sequential-thread".to_owned())
        .spawn(move || {
            run(
                job_name.clone(),
                &files,
                n_reduce,
                |phase| match phase {
                    MrPhase::MapPhase => {
                        for (i, f) in files.iter().enumerate() {
                            common_map(
                                &job_name,
                                i,
                                f,
                                n_reduce,
                                mapFunc);
                        }
                    }
                    MrPhase::ReducePhase => {
                        for i in 0..n_reduce {
                            common_reduce(
                                &job_name,
                                i,
                                &merge_name(&job_name, i),
                                files.len(),
                                reduceFunc);
                        }
                    }
                },
                seq_finish,
            )
        });
    handle.unwrap().join().expect("sequential join failed.");
}

#[allow(unused_variables)]
fn run<F>(
    job_name: String,
    files: &Vec<String>,
    n_reduce: usize,
    schedule: F,
    finish: fn(), )
    where F: Fn(MrPhase)
{
    info!("Start run: ");
    schedule(MrPhase::MapPhase);
    schedule(MrPhase::ReducePhase);
    finish();
    merge(&job_name, n_reduce);
    info!("run finish");
}


fn seq_finish() {
    info!("check the mrtmp:{{jobname}} file for result.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mr_start_without_env() {}
}