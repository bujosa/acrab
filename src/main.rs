use google_cloud_default::WithAuthExt;
use google_cloud_gax::grpc::Status;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    subscription::SubscriptionConfig,
};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Status> {
    publisher().await?;

    subscriber().await
}

async fn publisher() -> Result<(), Status> {
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config).await.unwrap();

    // Publish a message to a topic.
    let topic = client.topic("projects/curbo-dev/topics/example");
    if !topic.exists(None).await? {
        topic.create(None, None).await?;
    }

    // Start publisher.
    let publisher = topic.new_publisher(None);

    // Publish message.
    let tasks: Vec<JoinHandle<Result<String, Status>>> = (0..10)
        .into_iter()
        .map(|_i| {
            let publisher = publisher.clone();
            tokio::spawn(async move {
                let msg = PubsubMessage {
                    data: "Hola From Publisher".into(),
                    // Set ordering_key if needed (https://cloud.google.com/pubsub/docs/ordering)
                    ordering_key: "order".into(),
                    ..Default::default()
                };

                // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
                let awaiter = publisher.publish(msg).await;

                // The get method blocks until a server-generated ID or an error is returned for the published message.
                awaiter.get().await
            })
        })
        .collect();

    // Wait for all publish task finish
    for task in tasks {
        let message_id = task.await.unwrap()?;

        println!("Message ID: {}", message_id);
    }

    Ok(())
}

async fn subscriber() -> Result<(), Status> {
    println!("Subscriber Join");

    let config = ClientConfig::default().with_auth().await.unwrap();

    let client = Client::new(config).await.unwrap();

    // Get the topic to subscribe to.
    let topic = client.topic("projects/curbo-dev/topics/example");

    // Create subscription
    // If subscription name does not contain a "/", then the project is taken from client above. Otherwise, the
    // name will be treated as a fully qualified resource name
    let config = SubscriptionConfig {
        // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
        enable_message_ordering: true,
        ..Default::default()
    };

    // Create subscription
    let subscription = client.subscription("test-example-2");
    if !subscription.exists(None).await? {
        subscription
            .create(topic.fully_qualified_name(), config, None)
            .await?;
    }

    // Token for cancel.
    let cancel = CancellationToken::new();
    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        // Cancel after 10 seconds.
        tokio::time::sleep(Duration::from_secs(10)).await;
        cancel2.cancel();
    });

    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.
    subscription
        .receive(
            |mut message, cancel| async move {
                // Handle data.
                println!("Got Message: {:?}", message.message.data);

                // Ack or Nack message.
                let _ = message.ack().await;
            },
            cancel.clone(),
            None,
        )
        .await?;

    // Delete subscription if needed.
    subscription.delete(None).await?;

    Ok(())
}
