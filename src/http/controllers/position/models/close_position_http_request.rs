use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::{ApiResponseCodes, PositionSide, SlTpType};

#[derive(MyHttpInput)]
pub struct ClosePositionHttpRequest {
    #[http_form_data(name = "processId"; description = "Process id")]
    pub process_id: String,
    #[http_form_data(name = "accountId"; description = "Account id")]
    pub account_id: String,
    #[http_form_data(name = "positionId"; description = "Position id")]
    pub position_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ClosePositionHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<ClosedPositionApiModel>,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ClosedPositionApiModel {
    pub id: String,
    pub account_id: String,
    pub instrument: String,
    pub invest_amount: f64,
    pub multiplier: i32,
    pub open_price: f64,
    pub open_date: u64,
    pub operation: PositionSide,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub tp_type: Option<SlTpType>,
    pub sl_type: Option<SlTpType>,
    pub close_price: f64,
    pub close_date: u64,
    pub swap: f64,
    pub commissions: f64,
}
