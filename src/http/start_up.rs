use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use my_http_server_controllers::swagger::SwaggerMiddleware;
use rest_api_wl_shared::middlewares::AuthMiddleware;

use crate::app::AppContext;

pub fn setup_server(app: Arc<AppContext>, port: u16) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], port)));

    let controllers = Arc::new(super::builder::build_controllers(&app));

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        crate::app::APP_NAME.to_string(),
        crate::app::APP_VERSION.to_string(),
    );

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        crate::app::APP_NAME.to_string(),
        crate::app::APP_VERSION.to_string(),
    )));

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.add_middleware(Arc::new(AuthMiddleware::new(
        app.sessions_ns_reader.clone(),
    )));

    http_server.add_middleware(controllers);

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
