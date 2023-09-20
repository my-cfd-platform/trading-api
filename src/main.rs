use std::sync::Arc;

use app::AppContext;
use http::setup_server;

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
        setup_server(app_context.clone(), builder);
    });

    service_context.start_application().await;
}
