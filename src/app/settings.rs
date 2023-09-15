use service_sdk::my_no_sql::reader::MyNoSqlTcpConnectionSettings;
use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

use crate::TradingExecutorGrpcClient;

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "NoSqlTcp")]
    pub nosql_tcp: String,

    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,

    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,

    #[serde(rename = "TradingExecutorGrpcService")]
    pub trading_executor_grpc_service: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.nosql_tcp.clone()
    }
}

#[async_trait::async_trait]
impl service_sdk::my_telemetry::my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}

#[async_trait::async_trait]
impl service_sdk::my_logger::my_seq_logger::SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TradingExecutorGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.trading_executor_grpc_service.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
