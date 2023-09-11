use std::sync::Arc;

use crate::AppContext;
use my_http_server_controllers::controllers::{
    ControllersAuthorization, ControllersMiddleware, RequiredClaims,
};
use rest_api_wl_shared::middlewares::AuthFailResponseFactory;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(
        ControllersAuthorization::BearerAuthentication {
            global: true,
            global_claims: RequiredClaims::no_claims(),
        }
        .into(),
        Some(Arc::new(AuthFailResponseFactory)),
    );

    result.register_post_action(Arc::new(super::positions::OpenPositionHttpAction::new(
        app.clone(),
    )));
    result.register_post_action(Arc::new(
        super::positions::UpdateSlTpControllerHttpAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::positions::ClosePositionControllerHttpAction::new(app.clone()),
    ));

    result
}
