use my_http::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::ApiResponseCodes;

use super::*;

#[derive(MyHttpInput)]
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

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateTpSlHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<ActivePositionApiModel>,
}
