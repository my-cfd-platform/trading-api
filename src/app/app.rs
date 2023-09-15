use std::sync::Arc;

use service_sdk::my_no_sql::reader::{MyNoSqlDataReader, MyNoSqlTcpConnection, MyNoSqlDataReaderTcp};
use rest_api_wl_shared::middlewares::SessionEntity;
use rust_extensions::AppStates;

use crate::{SettingsReader, TradingExecutorGrpcClient};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub sessions_ns_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
    pub my_no_sql_connection: MyNoSqlTcpConnection,
    pub app_states: Arc<AppStates>,
    pub trading_executor_grpc_service: Arc<TradingExecutorGrpcClient>,
    pub debug: bool,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        let my_no_sql_connection = service_sdk::my_no_sql::reader::MyNoSqlTcpConnection::new(
            format!("{}:{}", crate::app::APP_NAME, crate::app::APP_VERSION),
            settings_reader.clone(),
        );

        Self {
            sessions_ns_reader: my_no_sql_connection.get_reader().await,
            my_no_sql_connection,
            app_states: Arc::new(AppStates::create_initialized()),
            trading_executor_grpc_service: Arc::new(TradingExecutorGrpcClient::new(
                settings_reader.clone(),
            )),
            debug: std::env::var("DEBUG").is_ok(),
        }
    }
}
