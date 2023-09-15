use std::sync::Arc;

use my_http::core::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{ApiResponseCodes, AppContext};

use crate::trading_executor_grpc::{
    TradingExecutorOperationsCodes, TradingExecutorUpdateSlTpGrpcRequest,
};

use super::*;

#[my_http::macros::http_route(
    method: "POST",
    route: "/api/trading/v1/Positions/UpdateTpSl",
    summary: "Update sl tp",
    description: "Update position sl tp",
    controller: "Positions",
    input_data: "UpdateSlTpHttpRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "UpdateTpSlHttpResponse"},
    ]
)]
pub struct UpdateSlTpControllerHttpAction {
    app: Arc<AppContext>,
}

impl UpdateSlTpControllerHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &UpdateSlTpControllerHttpAction,
    input_data: UpdateSlTpHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    let mut request = TradingExecutorUpdateSlTpGrpcRequest {
        position_id: input_data.position_id,
        account_id: input_data.account_id,
        trader_id: trader_id.to_string(),
        tp_in_profit: None,
        sl_in_profit: None,
        tp_in_asset_price: None,
        sl_in_asset_price: None,
        process_id: input_data.process_id,
    };

    if input_data.tp_type.is_some() {
        let tp = Some(input_data.tp.unwrap());
        match input_data.tp_type.unwrap() {
            SlTpType::Price => {
                request.tp_in_asset_price = tp;
            }
            SlTpType::Currency => {
                request.tp_in_profit = tp;
            }
            SlTpType::Percent => panic!("Percent tp tp not supported"),
        };
    }

    if input_data.sl_type.is_some() {
        let sl = Some(input_data.sl.unwrap());
        match input_data.tp_type.unwrap() {
            SlTpType::Price => {
                request.sl_in_asset_price = sl;
            }
            SlTpType::Currency => {
                request.sl_in_profit = sl;
            }
            SlTpType::Percent => panic!("Percent sl tp not supported"),
        };
    }

    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .update_sl_tp(request, &ctx.telemetry_context)
        .await
        .unwrap();

    let response = match grpc_response.position {
        Some(position) => UpdateTpSlHttpResponse {
            result: ApiResponseCodes::Ok,
            position: Some(position.into()),
        },
        None => {
            let te_status = TradingExecutorOperationsCodes::try_from(grpc_response.status).unwrap();

            UpdateTpSlHttpResponse {
                result: te_status.into(),
                position: None,
            }
        }
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
