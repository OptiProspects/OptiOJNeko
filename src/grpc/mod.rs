pub mod service;

pub mod judge_grpc_service {
    tonic::include_proto!("judge_grpc_service");
}
