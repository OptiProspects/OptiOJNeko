mod grpc;

use crate::grpc::judge_grpc_service::judge_grpc_service_server::JudgeGrpcServiceServer;
use crate::grpc::service::JudgeGrpcServiceImpl;
use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_level(true)
        .pretty()
        .init();

    info!("判题服务启动中...");

    let addr = "0.0.0.0:50051".parse()?;
    let my_service = JudgeGrpcServiceImpl::default();

    info!("监听地址: {}", addr);

    Server::builder()
        .add_service(JudgeGrpcServiceServer::new(my_service))
        .serve(addr)
        .await?;

    Ok(())
}
