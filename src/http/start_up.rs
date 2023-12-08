use std::sync::Arc;

use service_sdk::HttpServerBuilder;

use crate::app::AppContext;

pub fn register_controllers(app: Arc<AppContext>, builder: &mut HttpServerBuilder) {
    builder.register_post_action(super::positions::OpenPositionHttpAction::new(app.clone()));
    builder.register_post_action(super::positions::UpdateSlTpControllerHttpAction::new(
        app.clone(),
    ));
    builder.register_post_action(super::positions::ClosePositionControllerHttpAction::new(
        app.clone(),
    ));
    builder.register_post_action(super::orders::SetPendingOrderAction::new(app.clone()));
    builder.register_post_action(super::orders::CancelPendingOrderAction::new(app.clone()));
}
