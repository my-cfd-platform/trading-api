service_sdk::macros::use_grpc_client!();

#[my_grpc_extensions::client::generate_grpc_client(
    proto_file: "./proto/TradingExecutorGrpcService.proto",
    crate_ns: "crate::trading_executor_grpc",
    retries: 3,
    request_timeout_sec: 30,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct TradingExecutorGrpcClient;
