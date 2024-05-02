use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};
use rest_api_wl_shared::GetClientId;
use service_sdk::my_http_server::{self, HttpOutput};

use crate::{
    app::AppContext,
    http::orders::CancelPendingPositionHttpResponse,
    trading_executor_grpc::{
        TradingExecutorCancelPendingGrpcRequest, TradingExecutorOperationsCodes,
    },
};

use super::CancelPendingPositionHttpRequest;

#[service_sdk::my_http_server::macros::http_route(
    method: "POST",
    route: "/api/trading/v1/PendingOrders/Cancel",
    summary: "Cancel pending Order",
    description: "Cancel pending order",
    input_data: "CancelPendingPositionHttpRequest",
    controller: "Orders",
    
    result:[
        {status_code: 200, description: "Ok response", model: "CancelPendingPositionHttpResponse"},
    ]
)]
pub struct CancelPendingOrderAction {
    app: Arc<AppContext>,
}

impl CancelPendingOrderAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &CancelPendingOrderAction,
    input_data: CancelPendingPositionHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id()?;

    let request = TradingExecutorCancelPendingGrpcRequest {
        account_id: input_data.account_id,
        trader_id: trader_id.to_string(),
        position_id: input_data.position_id,
    };

    if action.app.debug {
        println!("grpc_request: {:?}", request);
    }
    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .cancel_pending_position(request, &ctx.telemetry_context)
        .await
        .unwrap();

    if action.app.debug {
        println!("grpc_response: {:?}", grpc_response);
    }

    let response = match grpc_response.position {
        Some(position) => CancelPendingPositionHttpResponse {
            result: grpc_response.status.into(),
            position: Some(position.into()),
        },
        None => {
            let status: Option<TradingExecutorOperationsCodes> =
                TradingExecutorOperationsCodes::try_from(grpc_response.status).ok();

            CancelPendingPositionHttpResponse {
                result: status.unwrap().into(),
                position: None,
            }
        }
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
