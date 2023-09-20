use service_sdk::my_http_server;
use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{
    app::AppContext, http::map_http_to_grpc_open_position,
    trading_executor_grpc::TradingExecutorOperationsCodes,
};

use super::*;

#[service_sdk::my_http_server::macros::http_route(
    method: "POST",
    route: "/api/trading/v1/Positions/Open",
    summary: "Open position",
    description: "Open client position",
    controller: "Positions",
    input_data: "OpenPositionHttpRequest",
    authorized: ["KYC"],
    result:[
        {status_code: 200, description: "Ok response", model: "OpenPositionHttpResponse"},
    ]
)]
pub struct OpenPositionHttpAction {
    app: Arc<AppContext>,
}

impl OpenPositionHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &OpenPositionHttpAction,
    input_data: OpenPositionHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    let request = map_http_to_grpc_open_position(&input_data, trader_id);

    if action.app.debug {
        println!("grpc_request: {:?}", request);
    }
    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .open_position(request, &ctx.telemetry_context)
        .await
        .unwrap();

    if action.app.debug {
        println!("grpc_response: {:?}", grpc_response);
    }

    let response = match grpc_response.positon {
        Some(position) => OpenPositionHttpResponse {
            result: grpc_response.status.into(),
            position: Some(position.into()),
        },
        None => {
            let status: Option<TradingExecutorOperationsCodes> =
                TradingExecutorOperationsCodes::try_from(grpc_response.status).ok();

            OpenPositionHttpResponse {
                result: status.unwrap().into(),
                position: None,
            }
        }
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
