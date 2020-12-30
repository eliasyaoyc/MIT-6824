use mr::master_server::{Master, MasterServer};
use mr::worker_client::WorkerClient;
use mr::master_client::MasterClient;
use mr::{Empty, WorkerAddr};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Response, Status, Request};
use crate::common_rpc::{validate_uri, worker_do_task,TaskArg};
use tokio::time::delay_for;
use std::time::Duration;
use futures::future::join_all;
use crate::{MrPhase, merge};
use tonic::transport::Server;
use log::info;

pub mod mr {
    tonic::include_proto!("mr");
}


struct MasterService {
    workers: Arc<Mutex<Vec<String>>>,
}

#[tonic::async_trait]
impl Master for MasterService {
    async fn register(&self, request: Request<WorkerAddr>) -> Result<Response<Empty>, Status> {
        info!("got a register request from {:?}", request);
        let mut workers = self.workers.lock().await;
        let addr = request.into_inner().addr;
        workers.push(addr.clone());
        info!("current workers: {:?}", workers);
        Ok(Response::new(Empty::default()))
    }

    async fn shutdown(&self, _: Request<Empty>) -> Result<Response<Empty>, Status> {
        info!("shutting down master server");
        let workers = self.workers.lock().await;
        for worker in workers.iter() {
            let mut worker_addr = format!("{}", worker);
            validate_uri(&mut worker_addr);
            let mut client = WorkerClient::connect(worker_addr).await.unwrap();
            match client.shutdown(Request::new(Empty::default())).await {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        std::process::exit(1);
    }
}

impl MasterService {
    pub fn new(workers: Arc<Mutex<Vec<String>>>) -> Self {
        MasterService {
            workers,
        }
    }
}

#[allow(non_snake_case)]
pub async fn run_master(
    job_name: String,
    files: Vec<String>,
    n_reduce: usize,
    master_addr: String, ) -> Result<(), Box<dyn std::error::Error>>
{
    let free_workers = Arc::new(Mutex::new(vec![]));
    let workers = free_workers.clone();

    let addr_for_finish = master_addr.clone();
    let handle = tokio::spawn(async move {
        delay_for(Duration::from_secs(5)).await;
        schedule(
            job_name.clone(),
            files.clone(),
            n_reduce,
            MrPhase::MapPhase,
            free_workers.clone(),
        ).await;

        schedule(
            job_name.clone(),
            files.clone(),
            n_reduce,
            MrPhase::ReducePhase,
            free_workers.clone(),
        ).await;
        merge(&job_name, n_reduce);
        finish(addr_for_finish).await;
    });


    let route_guide = MasterService::new(workers);

    let addr = master_addr.parse().unwrap();
    let svc = MasterServer::new(route_guide);
    let server_handle = tokio::spawn(async move {
        Server::builder().add_service(svc).serve(addr).await.expect("start master server failed");
    });

    join_all(vec![handle, server_handle]).await;
    Ok(())
}

async fn finish(addr: String) {
    let mut addr = addr;
    validate_uri(&mut addr);
    info!("finish {}", addr);

    let mut client = MasterClient::connect(addr).await.expect("connect master server failed");
    let _ = client.shutdown(Request::new(Empty::default())).await.expect("shutdown server failed");
    info!("finished");
}

#[allow(dead_code)]
async fn schedule(
    job_name: String,
    map_files: Vec<String>,
    n_reduce: usize,
    phase: MrPhase,
    free_workers: Arc<Mutex<Vec<String>>>, )
{
    let n_tasks: usize;
    let n_other: usize;

    match phase {
        MrPhase::MapPhase => {
            n_tasks = map_files.len();
            n_other = n_reduce;
        }
        MrPhase::ReducePhase => {
            n_tasks = n_reduce;
            n_other = map_files.len();
        }
    }
    info!("Schedule: {} {:?} tasks ({} I/Os)", n_tasks, phase, n_other);

    let mut handles = vec![];
    for i in 0..n_tasks {
        let phase = phase.clone();

        let phase_string;
        let file;
        match phase {
            MrPhase::MapPhase => {
                phase_string = String::from("map_phase");
                file = map_files[i].clone();
            }
            MrPhase::ReducePhase => {
                phase_string = String::from("reduce_phase");
                file = "".to_owned();
            }
        }
        let job_name = job_name.clone();

        let shared_workers = free_workers.clone();
        let handle = tokio::spawn(async move {
            let worker;
            loop {
                let mut arr = shared_workers.lock().await;
                let len = arr.len();
                if len < 1 {
                    drop(arr);
                    delay_for(Duration::from_secs(2)).await;
                    continue;
                }
                worker = arr.remove(len - 1);
                break;
            }
            info!("scheduling {} task to {}", file, worker);
            let arg = TaskArg {
                job_name,
                file,
                phase: phase_string,
                task_number: i as i32,
                num_other_phase: n_other as i32,
            };
            worker_do_task(&worker, arg).await.expect("worker do task failed");

            let mut arr = shared_workers.lock().await;
            arr.push(worker);
            info!("work finished");
        });

        handles.push(handle);
    }

    join_all(handles).await;
    info!("schedule finished");
}