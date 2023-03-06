use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_settings_reader::SettingsModel;
use serde::{Serialize, Deserialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "TradingExecutorGrpcService")]
    pub trading_executor_grpc_service: String,
    #[serde(rename = "NoSqlTcp")]
    pub nosql_tcp: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsModel {
    async fn get_host_port(&self) -> String {
        self.nosql_tcp.clone()
    }
}