use std::{env, net::SocketAddr, time::Instant};
use tokio::time::{sleep, Duration};
use tonic::{transport::Server, Request, Response, Status};

use proto::fl;

use fl::participant_server::{Participant, ParticipantServer};
use fl::{TrainRequest, TrainResponse};

#[derive(Default)]
struct ParticipantService;

#[tonic::async_trait]
impl Participant for ParticipantService {
    async fn train(
        &self,
        request: Request<TrainRequest>,
    ) -> Result<Response<TrainResponse>, Status> {
        let id = env::var("PARTICIPANT_ID").unwrap_or_else(|_| "unknown".into());
        let latency: u64 = env::var("NETWORK_LATENCY_MS")
            .unwrap_or_else(|_| "20".into())
            .parse()
            .unwrap_or(20);

        let start = Instant::now();

        // Prototype: simulate local FL computation.
        sleep(Duration::from_millis(100 + latency)).await;

        let response = TrainResponse {
            participant_id: id,
            local_loss: 0.20 + request.get_ref().round as f32 * 0.001,
            training_ms: start.elapsed().as_millis() as u64,
            update_bytes: 4096,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50052".parse()?;
    println!("Participant listening on {}", addr);
    Server::builder()
        .add_service(ParticipantServer::new(ParticipantService::default()))
        .serve(addr)
        .await?;
    Ok(())
}
