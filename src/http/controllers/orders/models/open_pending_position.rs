use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};

use crate::http::{positions::{PositionSide, SlTpType}, ApiResponseCodes};
use service_sdk::my_http_server;

#[derive(MyHttpInput)]
pub struct OpenLimitPositionHttpRequest {
    #[http_form_data(name = "processId"; description = "Process id")]
    pub process_id: String,
    #[http_form_data(name = "accountId"; description = "Account id")]
    pub account_id: String,
    #[http_form_data(name = "instrumentId"; description = "Instrument id")]
    pub instrument_id: String,
    #[http_form_data(name = "investmentAmount"; description = "Invest amount")]
    pub invest_amount: f64,
    #[http_form_data(name = "multiplier"; description = "Leverage ")]
    pub multiplier: i32,
    #[http_form_data(name = "operation"; description = "Position side ")]
    pub operation: PositionSide,
    #[http_form_data(name = "tp"; description = "Tp ")]
    pub tp: Option<f64>,
    #[http_form_data(name = "tpType"; description = "Tp type ")]
    pub tp_type: Option<SlTpType>,
    #[http_form_data(name = "sl"; description = "Sl ")]
    pub sl: Option<f64>,
    #[http_form_data(name = "slType"; description = "Sl type ")]
    pub sl_type: Option<SlTpType>,
    #[http_form_data(name = "desirePrice"; description = "Limit order execute desire price")]
    pub desire_price: f64
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct PendingPositionApiModel {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub instrument: String,
    #[serde(rename = "investmentAmount")]
    pub invest_amount: f64,
    pub operation: PositionSide,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    #[serde(rename = "tpType")]
    pub tp_type: Option<SlTpType>,
    #[serde(rename = "slType")]
    pub sl_type: Option<SlTpType>,
    #[serde(rename = "desirePrice")]
    pub desire_price: f64
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OpenPendingPositionHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<PendingPositionApiModel>,
}