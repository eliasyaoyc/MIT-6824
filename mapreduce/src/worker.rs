use std::sync::Mutex;
use tonic::{Response, Status, Request};
use crate::common_reduce::common_reduce;
use crate::{merge_name, reduce, map};
use crate::common_map::common_map;
use tonic::transport::Server;
use tokio::time::delay_for;
use std::time::Duration;
use log::info;

pub mod mr {
    tonic::include_proto!("mr");
}
use mr::master_client::MasterClient;
use mr::worker_server::{Worker, WorkerServer};
use mr::{DoTaskArg, Empty, WorkerAddr};

#[derive(Debug)]
pub struct WorkerService {
    concurrent: Mutex<usize>,
}

#[tonic::async_trait]
impl Worker for WorkerService {
    async fn do_task(&self, request: Request<DoTaskArg>) -> Result<Response<Empty>, Status> {
        let arg = request.into_inner();

        let mut nc = self.concurrent.lock().unwrap();
        *nc += 1;
        if *nc > 1 {
            panic!("more than one work sent concurrently to a single worker");
        }

        drop(nc);

        if &arg.phase == "map_phase" {
            common_map(
                &arg.job_name,
                arg.task_number as usize,
                &arg.file,
                arg.num_other_phase as usize,
                map,
            )
        } else {
            common_reduce(
                &arg.job_name,
                arg.task_number as usize,
                &merge_name(&arg.job_name, arg.task_number as usize),
                arg.num_other_phase as usize,
                reduce,
            );
        }

        let mut nc = self.concurrent.lock().unwrap();
        *nc -= 1;
        drop(nc);
        Ok(Response::new(Empty::default()))
    }

    async fn shutdown(&self, _: Request<Empty>) -> Result<Response<Empty>, Status> {
        info!("shutting down master server");
        std::process::exit(0x0111);
    }
}


async fn regist_to_master(master_addr: String, worker_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("worker register to {}",master_addr);
    let mut master_addr = master_addr;
    if !master_addr.starts_with("http"){
        master_addr = format!("http://{}", master_addr);
    }
    let mut client = MasterClient::connect(master_addr).await?;

    let response = client
        .register(
            Request::new(WorkerAddr{
                addr: worker_addr,
            })
        )
        .await?;

    info!("register response = {:?}", response);
    Ok(())
}

async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse().expect("Invalid worker addr");
    info!("Worker listening on: {}", addr);

    let route_guide = WorkerService {
        concurrent: Mutex::new(0),
    };

    let svc = WorkerServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

#[allow(unused_variables)]
pub async fn run_worker(master_addr: String, worker_addr: String){
    let addr = worker_addr.clone();

    let handle = tokio::spawn(async move {
        start_server(&addr).await.expect("start server failed");
    });

    delay_for(Duration::from_secs(5)).await;
    regist_to_master(master_addr.clone(), worker_addr.clone()).await.expect("register to master failed");

    handle.await.unwrap();
}
