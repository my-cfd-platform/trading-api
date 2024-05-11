use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};

use crate::http::ApiResponseCodes;
use service_sdk::my_http_server;

use super::PendingPositionApiModel;

#[derive(MyHttpInput)]
pub struct CancelPendingPositionHttpRequest {
    #[http_form_data(name = "processId"; description = "Process id")]
    pub process_id: String,
    #[http_form_data(name = "accountId"; description = "Account id")]
    pub account_id: String,
    #[http_form_data(name = "positionId"; description = "Position id to cancel")]
    pub position_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct CancelPendingPositionHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<PendingPositionApiModel>,
}
