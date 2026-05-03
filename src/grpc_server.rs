use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_server::{PaymentService, PaymentServiceServer},
    transaction_service_server::{TransactionService, TransactionServiceServer},
    chat_service_server::{ChatService, ChatServiceServer},
    PaymentRequest, PaymentResponse,
    TransactionRequest, TransactionResponse,
    ChatMessage,
};

// ===== PAYMENT SERVICE (Unary) =====
#[derive(Debug, Default)]
pub struct MyPaymentService;

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        let req = request.into_inner();
        println!("Payment request from user: {}, amount: {}", req.user_id, req.amount);

        let response = PaymentResponse { success: true };
        Ok(Response::new(response))
    }
}

// ===== TRANSACTION SERVICE (Server Streaming) =====
#[derive(Debug, Default)]
pub struct MyTransactionService;

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {
        let req = request.into_inner();
        println!("Transaction history request for user: {}", req.user_id);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let transactions = vec![
                TransactionResponse {
                    transaction_id: "TXN001".to_string(),
                    status: "success".to_string(),
                    amount: 100.0,
                    timestamp: "2024-01-01 10:00:00".to_string(),
                },
                TransactionResponse {
                    transaction_id: "TXN002".to_string(),
                    status: "success".to_string(),
                    amount: 250.0,
                    timestamp: "2024-01-02 11:00:00".to_string(),
                },
                TransactionResponse {
                    transaction_id: "TXN003".to_string(),
                    status: "failed".to_string(),
                    amount: 50.0,
                    timestamp: "2024-01-03 12:00:00".to_string(),
                },
            ];

            for txn in transactions {
                tx.send(Ok(txn)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// ===== CHAT SERVICE (Bi-directional Streaming) =====
#[derive(Debug, Default)]
pub struct MyChatService;

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(msg) = stream.message().await.unwrap_or(None) {
                println!("Received from {}: {}", msg.user_id, msg.message);
                let reply = ChatMessage {
                    user_id: "server".to_string(),
                    message: format!("Echo: {}", msg.message),
                };
                tx.send(Ok(reply)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// ===== MAIN SERVER =====
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Server running on {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(MyPaymentService::default()))
        .add_service(TransactionServiceServer::new(MyTransactionService::default()))
        .add_service(ChatServiceServer::new(MyChatService::default()))
        .serve(addr)
        .await?;

    Ok(())
}