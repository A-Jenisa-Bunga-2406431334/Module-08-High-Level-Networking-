use tonic::Request;
use tokio_stream::StreamExt;

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
    chat_service_client::ChatServiceClient,
    PaymentRequest, TransactionRequest, ChatMessage,
};

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {

    // ===== 1. UNARY - Payment =====
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    let payment_request = Request::new(PaymentRequest {
        user_id: "user123".to_string(),
        amount: 500.0,
    });

    let payment_response = payment_client.process_payment(payment_request).await?;
    println!("Payment response: {:?}", payment_response.into_inner());

    // ===== 2. SERVER STREAMING - Transaction =====
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;

    let transaction_request = Request::new(TransactionRequest {
        user_id: "user123".to_string(),
    });

    let mut stream = transaction_client
        .get_transaction_history(transaction_request)
        .await?
        .into_inner();

    println!("\nTransaction History:");
    while let Some(txn) = stream.next().await {
        let txn = txn?;
        println!(
            "  ID: {}, Status: {}, Amount: {}, Time: {}",
            txn.transaction_id, txn.status, txn.amount, txn.timestamp
        );
    }

    // ===== 3. BI-DIRECTIONAL STREAMING - Chat =====
    let mut chat_client = ChatServiceClient::connect("http://[::1]:50051").await?;

    let messages = vec![
        ChatMessage { user_id: "user123".to_string(), message: "Hello!".to_string() },
        ChatMessage { user_id: "user123".to_string(), message: "How are you?".to_string() },
        ChatMessage { user_id: "user123".to_string(), message: "Bye!".to_string() },
    ];

    let request = Request::new(tokio_stream::iter(messages));
    let mut chat_stream = chat_client.chat(request).await?.into_inner();

    println!("\nChat:");
    while let Some(msg) = chat_stream.next().await {
        let msg = msg?;
        println!("  {}: {}", msg.user_id, msg.message);
    }

    Ok(())
}