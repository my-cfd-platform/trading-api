use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{
    map_http_to_grpc_open_position, trading_executor_grpc::TradingExecutorOperationsCodes,
    AppContext, OpenPositionHttpRequest, OpenPositionHttpResponse,
};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/trading/v1/Orders/Set",
    summary: "Set pending Order",
    description: "Set pending order",
    controller: "Orders",
    authorized: ["KYC"],
    result:[
        {status_code: 200, description: "Ok response", model: "OpenPositionHttpResponse"},
    ]
)]
pub struct OpenPositionControllerHttpAction {
    app: Arc<AppContext>,
}

impl OpenPositionControllerHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &OpenPositionControllerHttpAction,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    todo!("Implement");
}
