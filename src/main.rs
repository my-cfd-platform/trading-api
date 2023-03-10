use std::sync::Arc;

use my_logger::MyLogger;
use trading_api::{SettingsModel, AppContext, setup_server};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsModel::load(".my-cfd").await;
    let settings_reader = Arc::new(settings_reader);

    let app = AppContext::new(settings_reader.clone()).await;

    let app = Arc::new(app);

    app.my_no_sql_connection.start(Arc::new(MyLogger::new())).await;

    setup_server(app.clone(), 8000);
    app.app_states.wait_until_shutdown().await;
}
