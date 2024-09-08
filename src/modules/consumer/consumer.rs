use std::sync::OnceLock;
use tracing::{info, error};
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

    let connection = match Connection::connect(uri, options).await {
        Ok(conn) => {
            info!("Connected to rabbitmq");
            conn
        },
        Err(err) => {
            error!("Connection lost: {:?}", err);
            return;
        }
    };

    let channel = match connection.create_channel().await {
        Ok(chan) => {
            info!("Created channel");
            chan
        },
        Err(err) => {
            error!("Failed to create channel: {:?}", err);
            return;
        }
    };

    let _queue = match channel
        .queue_declare(
            queue_name(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
    {
        Ok(queue) => {
            info!("Declared queue: {:?}", queue.name());
            queue
        },
        Err(err) => {
            error!("Failed to declare queue: {:?}", err);
            return;
        }
    };

   let consumer = match channel
        .basic_consume(
            queue_name(),
            "tag_foo",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
    {
        Ok(cons) => {
            info!("Started consumer");
            cons
        },
        Err(err) => {
            error!("Failed to start consumer: {:?}", err);
            return;
        }
    };
    
    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = match delivery {
            Ok(Some(delivery)) => delivery,
            Ok(None) => return, // Consumer was canceled
            Err(error) => {
                error!("Failed to consume queue message: {:?}", error);
                return;
            }
        };

        // Here we add any worker logic we want to
        info!("Received message: {:?}", delivery.data);
        
        // We don't have any error from processing yet, so we ack the message
        // later we will implement Result handling here for basic ack/noAck logic based on that
        if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
            error!("Failed to acknowledge message: {:?}", err);
        }
    });
   
}
