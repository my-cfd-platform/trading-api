use serde::{Deserialize, Serialize};
use service_sdk::my_grpc_extensions::GrpcClientSettings;

use crate::grpc_clients::TradingExecutorGrpcClient;
service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel,
    SdkSettingsTraits,
    AutoGenerateSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    pub my_no_sql_tcp_reader: String,
    pub my_telemetry: String,
    pub seq_conn_string: String,
    pub trading_executor_grpc_service: String,
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TradingExecutorGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.trading_executor_grpc_service.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
