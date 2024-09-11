use std::{fmt::Debug, usize};

use axum::{routing::post, Json, Router};
use rdkafka::{producer::{BaseProducer, BaseRecord, Producer}, ClientConfig};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zerocopy::{AsBytes, FromBytes};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Debug, AsBytes, Clone, Copy, FromBytes, FromZeroes)]
#[repr(transparent)]
struct CharArray<const N: usize>([u8; N]);

#[derive(Debug, AsBytes, Clone, Copy, FromBytes, FromZeroes)]
#[repr(transparent)]
struct Bool(u8);

impl<'de, const N: usize> Deserialize<'de> for CharArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the incoming data as a `String`
        let s: String = String::deserialize(deserializer)?;
        
        let mut chars = [0; N];

        // Copy characters from the string into the fixed-size array
        for (i, c) in s.chars().take(N).enumerate() {
            chars[i] = c as u8;
        }

        // Return the populated char array
        Ok(CharArray(chars))
    }
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = bool::deserialize(deserializer)?;

        Ok(Bool(b.as_bytes()[0]))
    }
}

impl Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            0 => serializer.serialize_bool(false),
            _ => serializer.serialize_bool(true),
        }
    }
}

impl<const N: usize> Serialize for CharArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Collect non-null characters into a string, ignoring `'\0'`
        let string = String::from_utf8_lossy(&self.0);
        // Serialize the string
        serializer.serialize_str(&string)
    }
}

#[derive(Serialize, Deserialize, Debug, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
struct Payload {
    operation_code: u32,
    name: CharArray<10>,
    email: CharArray<10>,
    order_id: u32,
    amount: u32,
    amo: Bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct A {
    a: u32
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/ping", post(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Json(payload): Json<Payload>) -> Json<Payload> {
    let bin = payload.as_bytes();

    // println!("{:?}", bin);

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "172.18.2.183:9092")
        .create()
        .expect("Producer creation failed");

    producer.send(BaseRecord::to("test").key("test").payload(bin)).unwrap();

    println!("{:?}", bin);

    producer.flush(None).unwrap();

    println!("Message sent to kafka");

    // let val = Payload::ref_from(bin).unwrap();

    // println!("{:?}", val);

    Json(payload)
}