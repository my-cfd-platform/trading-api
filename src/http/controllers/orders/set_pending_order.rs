use std::sync::Arc;

use rest_api_wl_shared::GetClientId;
use service_sdk::my_http_server;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};

use crate::app::AppContext;

#[service_sdk::my_http_server::macros::http_route(
    method: "POST",
    route: "/api/trading/v1/Orders/Set",
    summary: "Set pending Order",
    description: "Set pending order",
    controller: "Orders",
    authorized: ["KYC"],
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct SetPendingOrderAction {
    app: Arc<AppContext>,
}

impl SetPendingOrderAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SetPendingOrderAction,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    todo!("Implement");
}
