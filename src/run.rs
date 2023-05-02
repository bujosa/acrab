use google_cloud_gax::grpc::Status;

use crate::pubsub::{publisher::publisher, subscriber::subscriber};

pub async fn run() -> Result<(), Status> {
    publisher().await?;

    subscriber().await
}
