use my_http_server_swagger::{MyHttpInput, MyHttpIntegerEnum, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::ApiResponseCodes;

#[derive(Clone, Copy, Deserialize, Debug, Serialize, MyHttpIntegerEnum)]
pub enum SlTpType {
    #[http_enum_case(id="0"; value="i"; description="")]
    Currency,
    #[http_enum_case(id="1"; value="1"; description="")]
    Price,
    #[http_enum_case(id="2"; value="2"; description="")]
    Percent,
}

#[derive(Clone, Copy, Deserialize, Debug, Serialize, MyHttpIntegerEnum)]
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
    #[http_form_data(name = "investAmount"; description = "Invest amount")]
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
pub struct OpenPostionHttpResponse {
    pub result: ApiResponseCodes,
    pub position: Option<ActivePositionApiModel>,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ActivePositionApiModel {
    pub id: String,
    pub account_id: String,
    pub instrument: String,
    pub invest_amount: f64,
    pub open_price: f64,
    pub open_date: u64,
    pub operation: PositionSide,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub tp_type: Option<SlTpType>,
    pub sl_type: Option<SlTpType>,
}
