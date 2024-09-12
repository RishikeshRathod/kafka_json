use dotenv::dotenv;
use payloads::PingPayload;
use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message,
};
use zerocopy::FromBytes;
mod payloads;
mod types;

fn main() {
    dotenv().ok();

    let consumer: BaseConsumer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            std::env::var("KAFKA_SERVER_HOST").expect("KAFKA_SERVER_HOST is not set"),
        )
        .set(
            "group.id",
            std::env::var("KAFKA_GROUP_ID").expect("KAFKA_GROUP_ID is not set"),
        )
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");

    let topics = std::env::var("KAFKA_TOPICS").expect("KAFKA_TOPICS is not set");

    let topics = topics.split(",").collect::<Vec<&str>>();

    consumer.subscribe(&topics).unwrap();

    for message in &consumer {
        println!("Received from kafka");
        // Handle the message
        println!(
            "{:?}",
            PingPayload::ref_from(message.unwrap().detach().payload().unwrap())
        );
    }
}
