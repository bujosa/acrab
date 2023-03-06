use google_cloud_gax::cancel::CancellationToken;
use google_cloud_gax::grpc::Status;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::SubscriptionConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Status> {
    // Create pubsub client.
    let config = ClientConfig::default();
    let client = Client::new(config).await.unwrap();

    // Get the topic to subscribe to.
    let topic = client.topic("test-topic");

    // Configure subscription.
    let mut config = SubscriptionConfig::default();
    // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
    config.enable_message_ordering = true;

    // Create subscription
    // If subscription name does not contain a "/", then the project is taken from client above. Otherwise, the
    // name will be treated as a fully qualified resource name
    let subscription = client.subscription("test-subscription");
    if !subscription.exists(None, None).await? {
        subscription
            .create(topic.fully_qualified_name(), config, None, None)
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

    let _ = subscription
        .receive(
            |message, _cancell| async move {
                // Handle data.
                let data = message.message.data.as_slice();
                println!("{:?}", data);

                // Ack or Nack message.
                let _ = message.ack().await;
            },
            cancel.clone(),
            None,
        )
        .await;

    // Delete subscription if needed.
    let _ = subscription.delete(None, None).await;

    Ok(())
}
