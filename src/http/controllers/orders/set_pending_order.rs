use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};
use rest_api_wl_shared::GetClientId;
use service_sdk::my_http_server::{self, HttpOutput};

use crate::{
    app::AppContext,
    http::{map_http_to_grpc_open_pending, orders::OpenPendingPositionHttpResponse},
    trading_executor_grpc::TradingExecutorOperationsCodes,
};

use super::OpenLimitPositionHttpRequest;

#[service_sdk::my_http_server::macros::http_route(
    method: "POST",
    route: "/api/trading/v1/PendingOrders/Set",
    summary: "Set pending Order",
    description: "Set pending order",
    input_data: "OpenLimitPositionHttpRequest",
    controller: "Orders",
    authorized: ["KYC"],
    result:[
        {status_code: 200, description: "Ok response", model: "OpenPendingPositionHttpResponse"},
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
    input_data: OpenLimitPositionHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id()?;

    let request = map_http_to_grpc_open_pending(&input_data, trader_id);

    if action.app.debug {
        println!("grpc_request: {:?}", request);
    }
    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .set_pending_position(request, &ctx.telemetry_context)
        .await
        .unwrap();

    if action.app.debug {
        println!("grpc_response: {:?}", grpc_response);
    }

    let response = match grpc_response.position {
        Some(position) => OpenPendingPositionHttpResponse {
            result: grpc_response.status.into(),
            position: Some(position.into()),
        },
        None => {
            let status: Option<TradingExecutorOperationsCodes> =
                TradingExecutorOperationsCodes::try_from(grpc_response.status).ok();

            OpenPendingPositionHttpResponse {
                result: status.unwrap().into(),
                position: None,
            }
        }
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
