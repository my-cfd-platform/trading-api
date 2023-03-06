use my_http_server_swagger::MyHttpIntegerEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, MyHttpIntegerEnum)]
pub enum ApiResponseCodes {
    #[http_enum_case(id="0"; value="0"; description="")]
    DayOff,

    #[http_enum_case(id="1"; value="1"; description="")]
    MinOperationLotViolated,

    #[http_enum_case(id="2"; value="2"; description="")]
    MaxOperationLotViolated,

  
    #[http_enum_case(id="3"; value="3"; description="")]
    MaxPositionByInstrumentViolated,

    // /// <summary>
    // /// Not enough balance to perform operation
    // /// </summary>
    #[http_enum_case(id="4"; value="4"; description="")]
    InsufficientMargin,

    // /// <summary>
    // /// Bid or Ask is not found
    // /// </summary>
    #[http_enum_case(id="5"; value="5"; description="")]
    NoLiquidity,

    // /// <summary>
    // /// Position not found to perform operation
    // /// </summary>
    #[http_enum_case(id="6"; value="6"; description="")]
    PositionNotFound,

    // /// <summary>
    // /// Take profit is to close to the market
    // /// </summary>
    #[http_enum_case(id="7"; value="7"; description="")]
    TpIsTooClose,

    // /// <summary>
    // /// Sto loss is to close to the market
    // /// </summary>
    #[http_enum_case(id="8"; value="8"; description="")]
    SlIsTooClose,

    // /// <summary>
    // /// Pending order is not found to perform operation
    // /// </summary>
    #[http_enum_case(id="9"; value="9"; description="")]
    PendingOrderNotFound,

    // /// <summary>
    // /// Account not found
    // /// </summary>
    #[http_enum_case(id="10"; value="10"; description="")]
    AccountNotFound,

    // /// <summary>
    // /// Instrument not found
    // /// </summary>
    #[http_enum_case(id="11"; value="11"; description="")]
    InstrumentNotFound,

    // /// <summary>
    // /// Instrument can not be used at the moment
    // /// </summary>
    #[http_enum_case(id="12"; value="12"; description="")]
    InstrumentCanNotBeUsed,

    // /// <summary>
    // /// OperationIsNotPossibleDuringSwap
    // /// </summary>
    #[http_enum_case(id="13"; value="13"; description="")]
    OperationIsNotPossibleDuringSwap,

    // /// <summary>
    // ///
    // /// </summary>
    #[http_enum_case(id="14"; value="14"; description="")]
    MaxAmountPendingOrders,

    // /// <summary>
    // /// Technical Error
    // /// </summary>
    #[http_enum_case(id="15"; value="15"; description="")]
    TechnicalError,

    // /// <summary>
    // /// Multiplier Not Found
    // /// </summary>
    #[http_enum_case(id="16"; value="16"; description="")]
    MultiplierNotFound,

    // /// <summary>
    // /// Instrument can not be used at the moment
    // /// </summary>
    #[http_enum_case(id="17"; value="17"; description="")]
    MaximumAmountOfDemoAccount,

    // /// <summary>
    // /// Trading Disabled
    // /// </summary>
    #[http_enum_case(id="18"; value="18"; description="")]
    TradingDisabled,

    // /// <summary>
    // /// Max Open Positions Amount
    // /// </summary>
    #[http_enum_case(id="19"; value="19"; description="")]
    MaxOpenPositionsAmount,

    // /// <summary>
    // /// Ok
    // /// </summary>
    #[http_enum_case(id="20"; value="20"; description="")]
    Ok,

    #[http_enum_case(id="21"; value="21"; description="")]
    NotEnoughBalance,
}
