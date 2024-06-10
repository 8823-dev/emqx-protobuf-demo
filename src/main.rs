use prost::Message;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::{sleep, Duration};

mod message {
    // include のところで Error 消えない... OUT_DIR は取れている
    // `OUT_DIR` not set, enable "build scripts" to fix
    include!(concat!(env!("OUT_DIR"), "/demo.rs"));
}

#[tokio::main]
async fn main() {
    let mqtt_options = MqttOptions::new("demo-client", "localhost", 1883);
    let (client, _) = AsyncClient::new(mqtt_options, 10);

    tokio::spawn(async move {
        for i in 0..10 {
            let message = message::DemoMessage {
                device_id: format!("device-{}", i),
                value: i as i32,
            };
            let payload = message.encode_to_vec();
            client.publish("demo/topic", QoS::AtLeastOnce, false, payload).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });

    let mut stream = client.subscribe("demo/topic", QoS::AtLeastOnce).await.unwrap();

    for _ in 0..10 {
        let (_, payload) = stream.next().await.unwrap();
        let message = message::DemoMessage::decode(payload.as_ref()).unwrap();
        println!("Received message: {:?}", message);
    }
}