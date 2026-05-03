mod grpc_server;
mod grpc_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting gRPC Server...");

    tokio::spawn(async {
        grpc_server::run_server().await.unwrap();
    });

    // Tunggu server siap
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Running gRPC Client...");
    grpc_client::run_client().await?;

    Ok(())
}