mod payloads;
mod types;

use axum::{extract::State, routing::post, Json, Router};
use dotenv::dotenv;
use payloads::{PingPayload, PongPayload};
use rdkafka::{
    producer::{BaseProducer, BaseRecord, Producer},
    ClientConfig,
};
use std::sync::Arc;
use zerocopy::AsBytes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Kafka producer
    let producer: BaseProducer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            std::env::var("KAFKA_SERVER_HOST").expect("KAFKA_SERVER_HOST is not set"),
        )
        .create()
        .expect("Producer creation failed");

    let producer = Arc::new(producer);

    // build our application with a route
    let app = Router::new()
        .route("/ping", post(ping))
        .route("/pong", post(pong))
        .with_state(producer);

    // run it
    let listener =
        tokio::net::TcpListener::bind(std::env::var("API_HOST").expect("API_HOST is not set"))
            .await
            .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ping(
    State(producer): State<Arc<BaseProducer>>,
    Json(payload): Json<PingPayload>,
) -> Json<PingPayload> {
    let bin = payload.as_bytes();

    producer
        .send(BaseRecord::to("test").key("test").payload(bin))
        .unwrap();

    producer.flush(None).unwrap();

    println!("Sent to kafka");

    Json(payload)
}

async fn pong(
    State(producer): State<Arc<BaseProducer>>,
    Json(payload): Json<PongPayload>,
) -> Json<PongPayload> {
    let bin = payload.as_bytes();

    producer
        .send(BaseRecord::to("test").key("test").payload(bin))
        .unwrap();

    producer.poll(None);
    // producer.flush(None).unwrap();

    println!("Sent to kafka");

    Json(payload)
}
