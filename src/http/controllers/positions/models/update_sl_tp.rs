use serde::{Deserialize, Serialize};
use service_sdk::my_http_server;
use service_sdk::my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};

use crate::http::ApiResponseCodes;

use super::*;

#[derive(Clone, MyHttpInput)]
pub struct UpdateSlTpHttpRequest {
    #[http_form_data(name = "processId"; description = "Process id")]
    pub process_id: String,
    #[http_form_data(name = "accountId"; description = "Account id")]
    pub account_id: String,
    #[http_form_data(name = "positionId"; description = "Position id")]
    pub position_id: String,
    #[http_form_data(name = "tp"; description = "Tp")]
    pub tp: Option<f64>,
    #[http_form_data(name = "sl"; description = "Sl")]
    pub sl: Option<f64>,
    #[http_form_data(name = "tpType"; description = "Tp type")]
    pub tp_type: Option<SlTpType>,
    #[http_form_data(name = "slType"; description = "Sl type")]
    pub sl_type: Option<SlTpType>,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateTpSlHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<ActivePositionApiModel>,
}
