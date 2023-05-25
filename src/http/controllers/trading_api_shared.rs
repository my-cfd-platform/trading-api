use my_http_server_swagger::MyHttpIntegerEnum;
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, MyHttpIntegerEnum)]
#[repr(u8)]
pub enum ApiResponseCodes {
    #[http_enum_case(id="0"; value="0"; description="")]
    Ok,
    #[http_enum_case(id="1"; value="1"; description="")]
    DayOff,
    #[http_enum_case(id="2"; value="2"; description="")]
    MinOperationLotViolated,
    #[http_enum_case(id="3"; value="3"; description="")]
    MaxOperationLotViolated,
    #[http_enum_case(id="4"; value="4"; description="")]
    MaxPositionByInstrumentViolated,
    #[http_enum_case(id="5"; value="5"; description="")]
    InsufficientMargin,
    #[http_enum_case(id="6"; value="6"; description="")]
    NoLiquidity,
    #[http_enum_case(id="7"; value="7"; description="")]
    PositionNotFound,
    #[http_enum_case(id="8"; value="8"; description="")]
    TpIsTooClose,
    #[http_enum_case(id="9"; value="9"; description="")]
    SlIsTooClose,
    #[http_enum_case(id="10"; value="10"; description="")]
    PendingOrderNotFound,

    // /// <summary>
    // /// Account not found
    // /// </summary>
    #[http_enum_case(id="11"; value="11"; description="")]
    AccountNotFound,

    // /// <summary>
    // /// Instrument not found
    // /// </summary>
    #[http_enum_case(id="12"; value="12"; description="")]
    InstrumentNotFound,

    // /// <summary>
    // /// Instrument can not be used at the moment
    // /// </summary>
    #[http_enum_case(id="13"; value="13"; description="")]
    InstrumentCanNotBeUsed,

    // /// <summary>
    // /// OperationIsNotPossibleDuringSwap
    // /// </summary>
    #[http_enum_case(id="14"; value="14"; description="")]
    OperationIsNotPossibleDuringSwap,

    // /// <summary>
    // ///
    // /// </summary>
    #[http_enum_case(id="15"; value="15"; description="")]
    MaxAmountPendingOrders,

    // /// <summary>
    // /// Technical Error
    // /// </summary>
    #[http_enum_case(id="16"; value="16"; description="")]
    TechnicalError,

    // /// <summary>
    // /// Multiplier Not Found
    // /// </summary>
    #[http_enum_case(id="17"; value="17"; description="")]
    MultiplierNotFound,

    // /// <summary>
    // /// Instrument can not be used at the moment
    // /// </summary>
    #[http_enum_case(id="18"; value="18"; description="")]
    MaximumAmountOfDemoAccount,

    // /// <summary>
    // /// Trading Disabled
    // /// </summary>
    #[http_enum_case(id="19"; value="19"; description="")]
    TradingDisabled,

    // /// <summary>
    // /// Max Open Positions Amount
    // /// </summary>
    #[http_enum_case(id="20"; value="20"; description="")]
    MaxOpenPositionsAmount,

    #[http_enum_case(id="21"; value="21"; description="")]
    NotEnoughBalance,

    #[http_enum_case(id="22"; value="22"; description="")]
    TradingGroupNotFound,

    #[http_enum_case(id="23"; value="23"; description="")]
    TradingProfileNotFound,

    #[http_enum_case(id="24"; value="24"; description="")]
    TradingProfileInstrumentNotFound,
    #[http_enum_case(id="25"; value="25"; description="")]
    ExecutionReject,
}
