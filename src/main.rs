use std::sync::Arc;

use app::AppContext;
use http::register_controllers;
use service_sdk::ServiceInfo;

pub mod trading_executor_grpc {
    tonic::include_proto!("trading_executor");
}

mod app;
mod grpc_clients;
mod http;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = settings::SettingsReader::new(".my-cfd-platform").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app_context = Arc::new(AppContext::new(settings_reader.clone(), &service_context).await);

    service_context.configure_http_server(|builder| {
        rest_api_wl_shared::configure_rest_api_server(
            builder,
            app_context.sessions_ns_reader.clone(),
        );

        register_controllers(app_context.clone(), builder);
    });

    trade_log::core::TRADE_LOG
        .init_component_name(settings_reader.get_service_name().as_str())
        .await;
    trade_log::core::TRADE_LOG
        .start(&service_context.sb_client)
        .await;

    service_context.start_application().await;
}
