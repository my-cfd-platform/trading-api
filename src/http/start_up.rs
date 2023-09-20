use std::sync::Arc;

use rest_api_wl_shared::middlewares::{AuthFailResponseFactory, AuthMiddleware};
use service_sdk::HttpServerBuilder;

use crate::app::AppContext;

pub fn setup_server(app: Arc<AppContext>, builder: &mut HttpServerBuilder) {
    builder.set_auth_error_factory(AuthFailResponseFactory);

    builder.add_auth_middleware(Arc::new(AuthMiddleware::new(
        app.sessions_ns_reader.clone(),
    )));
    builder.register_post_action(super::positions::OpenPositionHttpAction::new(app.clone()));
    builder.register_post_action(super::positions::UpdateSlTpControllerHttpAction::new(
        app.clone(),
    ));
    builder.register_post_action(super::positions::ClosePositionControllerHttpAction::new(
        app.clone(),
    ));
    builder.register_post_action(super::orders::SetPendingOrderAction::new(app.clone()));
}
