use std::sync::OnceLock;
use std::env::var;

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};

fn queue_name() -> &'static str {
    static QUEUE_NAME: OnceLock<String> = OnceLock::new();
    QUEUE_NAME.get_or_init(|| {
        var("QUEUE_NAME").unwrap()
    })
}

fn amqp_uri() -> &'static str {
    static RABBITMQ_URI: OnceLock<String> = OnceLock::new();
    RABBITMQ_URI.get_or_init(|| {
        var("RABBITMQ_URI").unwrap()
    })
}

pub async fn consumer() {
    let uri = amqp_uri();
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(uri, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();

    let _queue = channel
        .queue_declare(
            queue_name(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let consumer = channel
        .basic_consume(
            queue_name(),
            "tag_foo",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = match delivery {
            // Carries the delivery alongside its channel
            Ok(Some(delivery)) => delivery,
            // The consumer got canceled
            Ok(None) => return,
            // Carries the error and is always followed by Ok(None)
            Err(error) => {
                dbg!("Failed to consume queue message {}", error);
                return;
            }
        };

        // Here we add any worker logic we want to
        println!("Received message: {:?}", delivery);

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to ack send_webhook_event message");
    });
}
