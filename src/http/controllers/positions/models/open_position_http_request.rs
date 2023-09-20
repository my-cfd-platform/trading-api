use service_sdk::my_http_server::macros::{MyHttpInput, MyHttpIntegerEnum, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};
use service_sdk::my_http_server;
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::http::ApiResponseCodes;


#[derive(Clone, Copy, Serialize_repr, Debug, Deserialize_repr, MyHttpIntegerEnum)]
#[repr(u8)]
pub enum SlTpType {
    #[http_enum_case(id="0"; value="i"; description="")]
    Currency,
    #[http_enum_case(id="1"; value="1"; description="")]
    Price,
    #[http_enum_case(id="2"; value="2"; description="")]
    Percent,
}

#[derive(Clone, Copy, Deserialize_repr, Debug, Serialize_repr, MyHttpIntegerEnum)]
#[repr(u8)]
pub enum PositionSide {
    #[http_enum_case(id="0"; value="0"; description="")]
    Buy,
    #[http_enum_case(id="1"; value="1"; description="")]
    Sell,
}

#[derive(MyHttpInput)]
pub struct OpenPositionHttpRequest {
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
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OpenPositionHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<ActivePositionApiModel>,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ActivePositionApiModel {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub instrument: String,
    #[serde(rename = "investmentAmount")]
    pub invest_amount: f64,
    #[serde(rename = "openPrice")]
    pub open_price: f64,
    #[serde(rename = "openDate")]
    pub open_date: u64,
    pub operation: PositionSide,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    #[serde(rename = "tpType")]
    pub tp_type: Option<SlTpType>,
    #[serde(rename = "slType")]
    pub sl_type: Option<SlTpType>,
}