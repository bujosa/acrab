use google_cloud_default::WithAuthExt;
use google_cloud_gax::grpc::Status;
use google_cloud_pubsub::client::{Client, ClientConfig};
use std::str;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub async fn subscriber() -> Result<(), Status> {
    let config = ClientConfig::default().with_auth().await.unwrap();

    let client = Client::new(config).await.unwrap();

    // Create subscription
    let subscription = client.subscription("projects/curbo-dev/subscriptions/test-subscriptions");

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
            |message, cancel| async move {
                let data = str::from_utf8(&message.message.data).unwrap();
                println!("Got Message: {}", data);

                // Ack or Nack message.
                let _ = message.ack().await;
            },
            cancel.clone(),
            None,
        )
        .await?;

    Ok(())
}
