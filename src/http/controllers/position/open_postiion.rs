use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_wl_shared::GetClientId;

use crate::{
    map_http_to_grpc_ppen_position, ApiResponseCodes, AppContext, OpenPositionHttpRequest,
    OpenPostionHttpResponse,
};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "api/v1/Positions/Open",
    summary: "Get client withdrawals",
    description: "Returns client withdrawals",
    controller: "ClientWithdrawal",
    input_data: "OpenPositionHttpRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "OpenPostionHttpResponse"},
    ]
)]
pub struct ClientWithdrawalControllerHttpAction {
    app: Arc<AppContext>,
}

impl ClientWithdrawalControllerHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ClientWithdrawalControllerHttpAction,
    input_data: OpenPositionHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let trader_id = ctx.get_client_id().unwrap();

    let request = map_http_to_grpc_ppen_position(&input_data, trader_id);

    let grpc_response = action
        .app
        .trading_executor_grpc_service
        .open_position(request)
        .await;

    let response = match grpc_response.positon {
        Some(position) => OpenPostionHttpResponse {
            result: ApiResponseCodes::Ok,
            position: Some(position.into()),
        },
        None => OpenPostionHttpResponse {
            result: ApiResponseCodes::Ok,
            position: None,
        },
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
