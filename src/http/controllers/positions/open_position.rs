use service_sdk::my_http_server;
use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{
    app::AppContext, http::{map_http_to_grpc_open_position, ApiResponseCodes},
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
    let trader_id = ctx.get_client_id()?;

    trade_log::trade_log!(
        trader_id,
        &input_data.account_id,
        &input_data.process_id,
        "n/a",
        "Got open position request.",
        ctx.telemetry_context.clone(),
        "input_data" = &input_data
    );

    let request = map_http_to_grpc_open_position(&input_data, trader_id);

    if let Some(result) = action.app.cache.get_and_track_empty::<OpenPositionHttpResponse>(&input_data.process_id).await{

        if let Some(cached_response) = result{
            trade_log::trade_log!(
                trader_id,
                &input_data.account_id,
                &input_data.process_id,
                "n/a",
                "Found request with same process id, but with response - returning old.",
                ctx.telemetry_context.clone(),
                "input_data" = &input_data,
                "response" = &cached_response
            );

            return HttpOutput::as_json(cached_response).into_ok_result(true);
        }else{
            trade_log::trade_log!(
                trader_id,
                &input_data.account_id,
                &input_data.process_id,
                "n/a",
                "Found request with same process id without response - returning error.",
                ctx.telemetry_context.clone(),
                "input_data" = &input_data
            );

            return HttpOutput::as_json(OpenPositionHttpResponse{
                result: ApiResponseCodes::ProcessIdDuplicate,
                position: None,
            }).into_ok_result(true);
        }
    }
    
    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .open_position(request, &ctx.telemetry_context)
        .await
        .unwrap();

    let response = match &grpc_response.position {
        Some(position) => OpenPositionHttpResponse {
            result: grpc_response.status.into(),
            position: Some(position.to_owned().into()),
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

    trade_log::trade_log!(
        trader_id,
        &input_data.account_id,
        &input_data.process_id,
        "n/a",
        "Got grpc response from engine",
        ctx.telemetry_context.clone(),
        "input_data" = &input_data,
        "grpc_response" = &grpc_response,
        "http_response" = &response
    );

    action.app.cache.set(&input_data.process_id, response.clone()).await;

    println!("response: {}", serde_json::to_string(&response).unwrap());

    HttpOutput::as_json(response).into_ok_result(true)
}