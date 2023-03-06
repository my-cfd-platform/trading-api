fn main() {
    tonic_build::compile_protos("proto/trading_executor_grpc_service.proto").unwrap();
}
