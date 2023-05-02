use google_cloud_default::WithAuthExt;
use google_cloud_gax::grpc::Status;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use tokio::task::JoinHandle;

pub async fn publisher() -> Result<(), Status> {
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
    let tasks: Vec<JoinHandle<Result<String, Status>>> = (0..1)
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
