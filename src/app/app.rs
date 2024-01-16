use std::sync::Arc;

use rest_api_wl_shared::middlewares::SessionEntity;
use rust_extensions::AppStates;
use service_sdk::{my_no_sql_sdk::reader::MyNoSqlDataReaderTcp, ServiceContext};

use crate::{grpc_clients::TradingExecutorGrpcClient, settings::SettingsReader, http::ProcessIdCache};

pub struct AppContext {
    pub sessions_ns_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
    pub app_states: Arc<AppStates>,
    pub trading_executor_grpc_service: Arc<TradingExecutorGrpcClient>,
    pub debug: bool,
    pub cache: ProcessIdCache
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>, service_ctx: &ServiceContext) -> Self {
        Self {
            sessions_ns_reader: service_ctx.get_ns_reader().await,
            app_states: Arc::new(AppStates::create_initialized()),
            trading_executor_grpc_service: Arc::new(TradingExecutorGrpcClient::new(
                settings_reader.clone(),
            )),
            debug: std::env::var("DEBUG").is_ok(),
            cache: ProcessIdCache::new()
        }
    }
}
