use std::sync::Arc;

use my_logger::{my_seq_logger::SeqLogger, MyLogger};
use trading_api::{setup_server, AppContext, SettingsReader, APP_NAME, APP_VERSION};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd").await;
    let settings_reader = Arc::new(settings_reader);

    SeqLogger::enable_from_connection_string(settings_reader.clone());

    my_logger::LOGGER
        .populate_app_and_version(APP_NAME, APP_VERSION)
        .await;

    let telemetry_writer = my_telemetry::my_telemetry_writer::MyTelemetryWriter::new(
        APP_NAME.to_string(),
        settings_reader.clone(),
    );

    let app = AppContext::new(settings_reader).await;

    let app = Arc::new(app);

    app.my_no_sql_connection
        .start(Arc::new(MyLogger::new()))
        .await;

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    setup_server(app.clone(), 8000);
    app.app_states.wait_until_shutdown().await;
}
