use std::sync::Arc;

use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use rest_api_wl_shared::middlewares::SessionEntity;
use rust_extensions::AppStates;

use crate::{SettingsModel, TradingExecutorGrpcClient};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub sessions_ns_reader: Arc<MyNoSqlDataReader<SessionEntity>>,
    pub my_no_sql_connection: MyNoSqlTcpConnection,
    pub app_states: Arc<AppStates>,
    pub trading_executor_grpc_service: Arc<TradingExecutorGrpcClient>,
}

impl AppContext {
    pub async fn new(settings: Arc<SettingsModel>) -> Self {

        let my_no_sql_connection = my_no_sql_tcp_reader::MyNoSqlTcpConnection::new(
            format!("{}:{}", crate::app::APP_NAME, crate::app::APP_VERSION),
            settings.clone(),
        );

        Self {
            sessions_ns_reader: my_no_sql_connection.get_reader().await,
            my_no_sql_connection,
            app_states: Arc::new(AppStates::create_initialized()),
            trading_executor_grpc_service: Arc::new(TradingExecutorGrpcClient::new(settings.trading_executor_grpc_service.clone()).await),
        }
    }
}
