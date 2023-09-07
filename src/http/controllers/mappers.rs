use crate::{
    trading_executor_grpc::{
        TradingExecutorActivePositionGrpcModel, TradingExecutorClosedPositionGrpcModel,
        TradingExecutorOpenPositionGrpcRequest, TradingExecutorOperationsCodes,
        TradingExecutorPositionSide,
    },
    ActivePositionApiModel, ApiResponseCodes, ClosedPositionApiModel, OpenPositionHttpRequest,
    PositionSide, SlTpType,
};

impl Into<PositionSide> for TradingExecutorPositionSide {
    fn into(self) -> PositionSide {
        match self {
            TradingExecutorPositionSide::Buy => PositionSide::Buy,
            TradingExecutorPositionSide::Sell => PositionSide::Sell,
        }
    }
}

impl Into<ActivePositionApiModel> for TradingExecutorActivePositionGrpcModel {
    fn into(self) -> ActivePositionApiModel {
        let side = TradingExecutorPositionSide::from_i32(self.side).unwrap();

        let mut model = ActivePositionApiModel {
            id: self.id,
            account_id: self.account_id,
            instrument: self.asset_pair,
            invest_amount: self.invest_amount,
            open_price: self.open_price,
            open_date: self.open_date,
            operation: side.into(),
            tp: None,
            sl: None,
            tp_type: None,
            sl_type: None,
        };

        if self.sl_in_asset_price.is_some() {
            model.sl_type = Some(SlTpType::Currency);
            model.sl = self.sl_in_asset_price;
        };

        if self.sl_in_profit.is_some() {
            model.sl_type = Some(SlTpType::Percent);
            model.sl = self.sl_in_profit;
        };

        if self.tp_in_asset_price.is_some() {
            model.tp_type = Some(SlTpType::Currency);
            model.tp = self.tp_in_asset_price;
        };

        if self.tp_in_profit.is_some() {
            model.tp_type = Some(SlTpType::Percent);
            model.tp = self.tp_in_profit;
        };

        return model;
    }
}

pub fn map_closed_grpc_to_api(
    response: TradingExecutorClosedPositionGrpcModel,
    account_id: &str,
) -> ClosedPositionApiModel {
    let mut http = ClosedPositionApiModel {
        id: response.id,
        account_id: account_id.to_string(),
        instrument: response.asset_pair,
        invest_amount: response.invest_amount,
        open_price: response.open_price,
        open_date: response.open_date,
        operation: response.side.into(),
        tp: None,
        sl: None,
        tp_type: None,
        sl_type: None,
        multiplier: response.leverage,
        close_price: response.close_price,
        close_date: response.open_date,
        swap: 0.0,
        commissions: 0.0,
    };

    if response.sl_in_asset_price.is_some() {
        http.sl_type = Some(SlTpType::Currency);
        http.sl = response.sl_in_asset_price;
    };

    if response.sl_in_profit.is_some() {
        http.sl_type = Some(SlTpType::Percent);
        http.sl = response.sl_in_profit;
    };

    if response.tp_in_asset_price.is_some() {
        http.tp_type = Some(SlTpType::Currency);
        http.tp = response.tp_in_asset_price;
    };

    if response.tp_in_profit.is_some() {
        http.tp_type = Some(SlTpType::Percent);
        http.tp = response.tp_in_profit;
    };

    return http;
}

pub fn map_http_to_grpc_open_position(
    open_http_request: &OpenPositionHttpRequest,
    trader_id: &str,
) -> TradingExecutorOpenPositionGrpcRequest {
    let mut open_request = TradingExecutorOpenPositionGrpcRequest {
        asset_pair: open_http_request.instrument_id.clone(),
        side: open_http_request.operation as i32,
        invest_amount: open_http_request.invest_amount,
        leverage: open_http_request.multiplier,
        process_id: open_http_request.process_id.clone(),
        tp_in_profit: None,
        sl_in_profit: None,
        tp_in_asset_price: None,
        sl_in_asset_price: None,
        account_id: open_http_request.account_id.clone(),
        trader_id: trader_id.to_string(),
    };

    if open_http_request.tp_type.is_some() {
        match open_http_request.tp_type.unwrap() {
            SlTpType::Currency => {
                open_request.tp_in_asset_price = open_http_request.tp;
            }
            SlTpType::Percent => {
                open_request.tp_in_profit = open_http_request.tp;
            }
            SlTpType::Price => panic!("Price not implemented"),
        };
    };

    if open_http_request.sl_type.is_some() {
        match open_http_request.sl_type.unwrap() {
            SlTpType::Currency => {
                open_request.sl_in_asset_price = open_http_request.sl;
            }
            SlTpType::Percent => {
                open_request.sl_in_profit = open_http_request.sl;
            }
            SlTpType::Price => panic!("Price not implemented"),
        };
    };

    return open_request;
}

impl Into<ApiResponseCodes> for TradingExecutorOperationsCodes {
    fn into(self) -> ApiResponseCodes {
        match self {
            TradingExecutorOperationsCodes::Ok => ApiResponseCodes::Ok,
            TradingExecutorOperationsCodes::DayOff => ApiResponseCodes::DayOff,
            TradingExecutorOperationsCodes::OperationIsTooLow => {
                ApiResponseCodes::MinOperationLotViolated
            }
            TradingExecutorOperationsCodes::OperationIsTooHigh => {
                ApiResponseCodes::MaxOperationLotViolated
            }
            TradingExecutorOperationsCodes::MinOperationsByInstrumentViolated => {
                ApiResponseCodes::MinOperationLotViolated
            }
            TradingExecutorOperationsCodes::MaxOperationsByInstrumentViolated => {
                ApiResponseCodes::MaxOperationLotViolated
            }
            TradingExecutorOperationsCodes::NotEnoughBalance => ApiResponseCodes::NotEnoughBalance,
            TradingExecutorOperationsCodes::NoLiquidity => ApiResponseCodes::NoLiquidity,
            TradingExecutorOperationsCodes::PositionNotFound => ApiResponseCodes::PositionNotFound,
            TradingExecutorOperationsCodes::TpIsTooClose => ApiResponseCodes::TpIsTooClose,
            TradingExecutorOperationsCodes::SlIsTooClose => ApiResponseCodes::SlIsTooClose,
            TradingExecutorOperationsCodes::AccountNotFound => ApiResponseCodes::AccountNotFound,
            TradingExecutorOperationsCodes::InstrumentNotFound => {
                ApiResponseCodes::InstrumentNotFound
            }
            TradingExecutorOperationsCodes::InstrumentIsNotTradable => {
                ApiResponseCodes::InstrumentNotFound
            }
            TradingExecutorOperationsCodes::HitMaxAmountOfPendingOrders => {
                ApiResponseCodes::MaxAmountPendingOrders
            }
            TradingExecutorOperationsCodes::TechError => ApiResponseCodes::TechnicalError,
            TradingExecutorOperationsCodes::MultiplierIsNotFound => {
                ApiResponseCodes::MultiplierNotFound
            }
            TradingExecutorOperationsCodes::TradingDisabled => ApiResponseCodes::TradingDisabled,
            TradingExecutorOperationsCodes::MaxPositionsAmount => {
                ApiResponseCodes::MaxOpenPositionsAmount
            }
            TradingExecutorOperationsCodes::TradingGroupNotFound => {
                ApiResponseCodes::TradingGroupNotFound
            }
            TradingExecutorOperationsCodes::TradingProfileNotFound => {
                ApiResponseCodes::TradingProfileNotFound
            }
            TradingExecutorOperationsCodes::TradingProfileInstrumentNotFound => {
                ApiResponseCodes::TradingProfileInstrumentNotFound
            }
            TradingExecutorOperationsCodes::ABookReject => ApiResponseCodes::ExecutionReject,
        }
    }
}

pub fn map_trading_executor_error_into_http_error(trading_executor_error: i32) -> ApiResponseCodes {
    match trading_executor_error {
        0 => ApiResponseCodes::Ok,
        1 => ApiResponseCodes::DayOff,
        2 => ApiResponseCodes::MinOperationLotViolated,
        3 => ApiResponseCodes::MaxOperationLotViolated,
        4 => ApiResponseCodes::MaxPositionByInstrumentViolated,
        5 => ApiResponseCodes::MaxPositionByInstrumentViolated,
        6 => ApiResponseCodes::NotEnoughBalance,
        7 => ApiResponseCodes::NoLiquidity,
        8 => ApiResponseCodes::PositionNotFound,
        9 => ApiResponseCodes::TpIsTooClose,
        10 => ApiResponseCodes::SlIsTooClose,
        11 => ApiResponseCodes::AccountNotFound,
        12 => ApiResponseCodes::InstrumentNotFound,
        13 => ApiResponseCodes::TradingDisabled,
        14 => ApiResponseCodes::MaxAmountPendingOrders,
        15 => ApiResponseCodes::TechnicalError,
        16 => ApiResponseCodes::MultiplierNotFound,
        17 => ApiResponseCodes::TradingDisabled,
        18 => ApiResponseCodes::MaxOpenPositionsAmount,
        19 => ApiResponseCodes::TechnicalError,
        20 => ApiResponseCodes::TechnicalError,
        21 => ApiResponseCodes::NotEnoughBalance,
        _ => panic!("Unknown error code"),
    }
}
