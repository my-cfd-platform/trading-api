use std::sync::Arc;

use crate::{AppContext, ClosePositionControllerHttpAction, OpenPositionControllerHttpAction, UpdateSlTpControllerHttpAction};
use my_http_server_controllers::controllers::{
    ControllersAuthorization, ControllersMiddleware, RequiredClaims,
};

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(
        ControllersAuthorization::BearerAuthentication {
            global: true,
            global_claims: RequiredClaims::no_claims(),
        }
        .into(),
        None,
    );

    result.register_post_action(Arc::new(OpenPositionControllerHttpAction::new(app.clone())));
    result.register_post_action(Arc::new(UpdateSlTpControllerHttpAction::new(app.clone())));
    result.register_post_action(Arc::new(ClosePositionControllerHttpAction::new(
        app.clone(),
    )));

    result
}
