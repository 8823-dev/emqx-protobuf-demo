use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::time::Duration;
use prost::Message;
use rand::Rng;

mod message {
    // include のところで Error 消えない... OUT_DIR は取れている
    // `OUT_DIR` not set, enable "build scripts" to fix
    include!(concat!(env!("OUT_DIR"), "/demo.rs"));
}

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("demo-client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("demo/topic", QoS::AtMostOnce).await.unwrap();

    task::spawn(async move {
        for i in 0..10 {
            let message = message::DemoMessage {
                device_id: format!("device-{}", i),
                value: rand::thread_rng().gen_range(1..=100),
            };
            let payload = message.encode_to_vec();
            println!("Published message (raw): {:?}", message);
            println!("Published message (encoded): {:?}", payload);

            let decoded_message = match message::DemoMessage::decode(&payload[..]) {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Failed to decode message: {:?}", e);
                    return;
                }
            };
            println!("Published message (decoded): {:?}", decoded_message);

            client.publish("demo/topic", QoS::AtLeastOnce, false, payload).await.unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {
        let notification = eventloop.poll().await.unwrap();
        println!("Received: {:?}", notification);
    }
}