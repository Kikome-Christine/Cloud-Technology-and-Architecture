use std::env;
use std::time::Instant;
use tonic::transport::Channel;

use proto::fl;

use fl::{
    scheduler_client::SchedulerClient,
    participant_client::ParticipantClient,
    ParticipantRequest, TrainRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Regulatory FL Orchestrator ===");

    let scheduler_url =
        env::var("SCHEDULER_URL").unwrap_or_else(|_| "http://scheduler:50051".into());

    let mut scheduler = SchedulerClient::connect(scheduler_url).await?;

    for round in 1..=3 {
        let round_start = Instant::now();

        let selection = scheduler
            .select_participants(ParticipantRequest {
                required_participants: 2,
            })
            .await?
            .into_inner();

        println!(
            "Round {}: {} participants selected",
            round,
            selection.participants.len()
        );

        let mut total_bytes = 0u64;
        let mut losses = Vec::new();

        for p in selection.participants {
            let host = match p.id.as_str() {
                "bank-a" => "participant1:50052",
                "bank-b" => "participant2:50052",
                _ => continue,
            };

            let channel = Channel::from_shared(format!("http://{}", host))?
                .connect()
                .await?;

            let mut client = ParticipantClient::new(channel);

            let result = client
                .train(TrainRequest { round })
                .await?
                .into_inner();

            println!(
                "  {} trained in {} ms, loss={:.3}, update={} bytes",
                result.participant_id,
                result.training_ms,
                result.local_loss,
                result.update_bytes
            );

            total_bytes += result.update_bytes;
            losses.push(result.local_loss);
        }

        let avg_loss = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };

        println!(
            "Round {} complete: latency={} ms, communication={} bytes, avg_loss={:.3}",
            round,
            round_start.elapsed().as_millis(),
            total_bytes,
            avg_loss
        );
    }

    Ok(())
}
