use google_cloud_gax::grpc::Status;

#[tokio::main]
async fn main() -> Result<(), Status> {
    acrab::run().await
}
