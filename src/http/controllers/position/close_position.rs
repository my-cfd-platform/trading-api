use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{
    map_closed_grpc_to_api,
    trading_executor_grpc::{
        TradingExecutorClosePositionGrpcRequest, TradingExecutorOperationsCodes,
    },
    ApiResponseCodes, AppContext, ClosePositionHttpRequest, ClosePositionHttpResponse,
};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/trading/v1/Positions/Close",
    summary: "Close position",
    description: "Close client position",
    controller: "Positions",
    input_data: "ClosePositionHttpRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "ClosePositionHttpResponse"},
    ]
)]
pub struct ClosePositionControllerHttpAction {
    app: Arc<AppContext>,
}

impl ClosePositionControllerHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ClosePositionControllerHttpAction,
    input_data: ClosePositionHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    let request = TradingExecutorClosePositionGrpcRequest {
        position_id: input_data.position_id,
        process_id: input_data.process_id,
        account_id: input_data.account_id.clone(),
        trader_id: trader_id.to_string(),
    };

    if action.app.debug {
        println!("request: {:?}", request);
    }

    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .close_position(request, &ctx.telemetry_context)
        .await
        .unwrap();

    if action.app.debug {
        println!("grpc_response: {:?}", grpc_response);
    }
    let response = match grpc_response.position {
        Some(position) => ClosePositionHttpResponse {
            result: ApiResponseCodes::Ok,
            position: Some(map_closed_grpc_to_api(position, &input_data.account_id)),
        },
        None => {
            let te_status = TradingExecutorOperationsCodes::try_from(grpc_response.status).unwrap();

            ClosePositionHttpResponse {
                result: te_status.into(),
                position: None,
            }
        }
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
