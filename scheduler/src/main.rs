use std::{env, net::SocketAddr};
use tonic::{transport::Server, Request, Response, Status};

use proto::fl;

use fl::scheduler_server::{Scheduler, SchedulerServer};
use fl::{ParticipantInfo, ParticipantRequest, ParticipantResponse};

#[derive(Default)]
struct SchedulerService;

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    async fn select_participants(
        &self,
        _request: Request<ParticipantRequest>,
    ) -> Result<Response<ParticipantResponse>, Status> {
        let urls = env::var("PARTICIPANT_URLS").unwrap_or_default();
        let mut selected = Vec::new();

        // In this prototype, participant metadata is represented by environment-backed
        // configuration. The scheduler applies the project's regulatory/resource rules.
        for (i, _url) in urls.split(',').enumerate() {
            let (id, cpu, memory, bandwidth, latency, eligible) = match i {
                0 => ("bank-a", 4, 8, 100, 20, true),
                1 => ("bank-b", 2, 4, 20, 80, true),
                _ => ("institution-c", 1, 2, 5, 250, false),
            };

            if eligible && cpu >= 2 && memory >= 2 && bandwidth >= 10 {
                selected.push(ParticipantInfo {
                    id: id.into(),
                    cpu_cores: cpu,
                    memory_gb: memory,
                    bandwidth_mbps: bandwidth,
                    latency_ms: latency,
                    regulatory_eligible: eligible,
                    available: true,
                });
            }
        }

        println!("Selected {} eligible participants", selected.len());
        Ok(Response::new(ParticipantResponse { participants: selected }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    println!("Scheduler listening on {}", addr);
    Server::builder()
        .add_service(SchedulerServer::new(SchedulerService::default()))
        .serve(addr)
        .await?;
    Ok(())
}
