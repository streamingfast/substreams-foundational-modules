// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub pyth_contract: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleInfo {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration="OracleType", tag="2")]
    pub oracle_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainlinkPriceState {
    #[prost(string, tag="1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
    #[prost(message, optional, tag="4")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandPriceState {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rate: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub resolve_time: u64,
    #[prost(uint64, tag="4")]
    pub request_id: u64,
    #[prost(message, optional, tag="5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedState {
    #[prost(string, tag="1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub quote: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub price_state: ::core::option::Option<PriceState>,
    #[prost(string, repeated, tag="4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderInfo {
    #[prost(string, tag="1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderState {
    #[prost(message, optional, tag="1")]
    pub provider_info: ::core::option::Option<ProviderInfo>,
    #[prost(message, repeated, tag="2")]
    pub provider_price_states: ::prost::alloc::vec::Vec<ProviderPriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderPriceState {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedInfo {
    #[prost(string, tag="1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub quote: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedPrice {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinbasePriceState {
    /// kind should always be "prices"
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    /// timestamp of the when the price was signed by coinbase
    #[prost(uint64, tag="2")]
    pub timestamp: u64,
    /// the symbol of the price, e.g. BTC
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    /// the value of the price scaled by 1e6
    #[prost(uint64, tag="4")]
    pub value: u64,
    /// the price state
    #[prost(message, optional, tag="5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceState {
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythPriceState {
    #[prost(string, tag="1")]
    pub price_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ema_price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ema_conf: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub conf: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub publish_time: u64,
    #[prost(message, optional, tag="6")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandOracleRequest {
    /// Unique Identifier for band ibc oracle request
    #[prost(uint64, tag="1")]
    pub request_id: u64,
    /// OracleScriptID is the unique identifier of the oracle script to be
    /// executed.
    #[prost(int64, tag="2")]
    pub oracle_script_id: i64,
    /// Symbols is the list of symbols to prepare in the calldata
    #[prost(string, repeated, tag="3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AskCount is the number of validators that are requested to respond to this
    /// oracle request. Higher value means more security, at a higher gas cost.
    #[prost(uint64, tag="4")]
    pub ask_count: u64,
    /// MinCount is the minimum number of validators necessary for the request to
    /// proceed to the execution phase. Higher value means more security, at the
    /// cost of liveness.
    #[prost(uint64, tag="5")]
    pub min_count: u64,
    /// FeeLimit is the maximum tokens that will be paid to all data source
    /// providers.
    #[prost(message, repeated, tag="6")]
    pub fee_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// PrepareGas is amount of gas to pay to prepare raw requests
    #[prost(uint64, tag="7")]
    pub prepare_gas: u64,
    /// ExecuteGas is amount of gas to reserve for executing
    #[prost(uint64, tag="8")]
    pub execute_gas: u64,
    /// MinSourceCount is the minimum number of data sources that must be used by
    /// each validator
    #[prost(uint64, tag="9")]
    pub min_source_count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandIbcParams {
    /// true if Band IBC should be enabled
    #[prost(bool, tag="1")]
    pub band_ibc_enabled: bool,
    /// block request interval to send Band IBC prices
    #[prost(int64, tag="2")]
    pub ibc_request_interval: i64,
    /// band IBC source channel
    #[prost(string, tag="3")]
    pub ibc_source_channel: ::prost::alloc::string::String,
    /// band IBC version
    #[prost(string, tag="4")]
    pub ibc_version: ::prost::alloc::string::String,
    /// band IBC portID
    #[prost(string, tag="5")]
    pub ibc_port_id: ::prost::alloc::string::String,
    ///   legacy oracle scheme ids
    #[prost(int64, repeated, tag="6")]
    pub legacy_oracle_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolPriceTimestamp {
    #[prost(enumeration="OracleType", tag="1")]
    pub oracle: i32,
    #[prost(string, tag="2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceTimestamps {
    #[prost(message, repeated, tag="1")]
    pub last_price_timestamps: ::prost::alloc::vec::Vec<SymbolPriceTimestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecords {
    #[prost(enumeration="OracleType", tag="1")]
    pub oracle: i32,
    #[prost(string, tag="2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub latest_price_records: ::prost::alloc::vec::Vec<PriceRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecord {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
}
/// MetadataStatistics refers to the metadata summary statistics of the
/// historical sample considered
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataStatistics {
    /// GroupCount refers to the number of groups used. Equals RecordsSampleSize if
    /// no grouping is used
    #[prost(uint32, tag="1")]
    pub group_count: u32,
    /// RecordsSampleSize refers to the total number of records used.
    #[prost(uint32, tag="2")]
    pub records_sample_size: u32,
    /// Mean refers to the arithmetic mean
    /// For trades, the mean is the VWAP computed over the grouped trade records ∑
    /// (price * quantity) / ∑ quantity For oracle prices, the mean is computed
    /// over the price records ∑ (price) / prices_count
    #[prost(string, tag="3")]
    pub mean: ::prost::alloc::string::String,
    /// TWAP refers to the time-weighted average price which equals ∑ (price_i *
    /// ∆t_i) / ∑ ∆t_i where ∆t_i = t_i - t_{i-1}
    #[prost(string, tag="4")]
    pub twap: ::prost::alloc::string::String,
    /// FirstTimestamp is the timestamp of the oldest record considered
    #[prost(int64, tag="5")]
    pub first_timestamp: i64,
    /// LastTimestamp is the timestamp of the youngest record considered
    #[prost(int64, tag="6")]
    pub last_timestamp: i64,
    /// MinPrice refers to the smallest individual raw price considered
    #[prost(string, tag="7")]
    pub min_price: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(string, tag="8")]
    pub max_price: ::prost::alloc::string::String,
    /// MedianPrice refers to the median individual raw price considered
    #[prost(string, tag="9")]
    pub median_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceAttestation {
    #[prost(string, tag="1")]
    pub price_id: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(int64, tag="2")]
    pub price: i64,
    #[prost(uint64, tag="3")]
    pub conf: u64,
    #[prost(int32, tag="4")]
    pub expo: i32,
    #[prost(int64, tag="5")]
    pub ema_price: i64,
    #[prost(uint64, tag="6")]
    pub ema_conf: u64,
    #[prost(int32, tag="7")]
    pub ema_expo: i32,
    #[prost(int64, tag="8")]
    pub publish_time: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OracleType {
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    Api3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIbc = 10,
    Provider = 11,
}
impl OracleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OracleType::Unspecified => "Unspecified",
            OracleType::Band => "Band",
            OracleType::PriceFeed => "PriceFeed",
            OracleType::Coinbase => "Coinbase",
            OracleType::Chainlink => "Chainlink",
            OracleType::Razor => "Razor",
            OracleType::Dia => "Dia",
            OracleType::Api3 => "API3",
            OracleType::Uma => "Uma",
            OracleType::Pyth => "Pyth",
            OracleType::BandIbc => "BandIBC",
            OracleType::Provider => "Provider",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Band" => Some(Self::Band),
            "PriceFeed" => Some(Self::PriceFeed),
            "Coinbase" => Some(Self::Coinbase),
            "Chainlink" => Some(Self::Chainlink),
            "Razor" => Some(Self::Razor),
            "Dia" => Some(Self::Dia),
            "API3" => Some(Self::Api3),
            "Uma" => Some(Self::Uma),
            "Pyth" => Some(Self::Pyth),
            "BandIBC" => Some(Self::BandIbc),
            "Provider" => Some(Self::Provider),
            _ => None,
        }
    }
}
/// MsgRelayProviderPrice defines a SDK message for setting a price through the
/// provider oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayProviderPrices {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub prices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayProviderPricesResponse {
}
/// MsgRelayPriceFeedPrice defines a SDK message for setting a price through the
/// pricefeed oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPriceFeedPrice {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub base: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub quote: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// price defines the price of the oracle base and quote
    #[prost(string, repeated, tag="4")]
    pub price: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPriceFeedPriceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayBandRates {
    #[prost(string, tag="1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, tag="3")]
    pub rates: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="4")]
    pub resolve_times: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="5")]
    pub request_i_ds: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayBandRatesResponse {
}
/// MsgRelayCoinbaseMessages defines a SDK message for relaying price messages
/// from Coinbase API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayCoinbaseMessages {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="2")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayCoinbaseMessagesResponse {
}
/// MsgRequestBandIBCRates defines a SDK message for requesting data from
/// BandChain using IBC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBandIbcRates {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub request_id: u64,
}
/// MsgRequestDataResponse defines the Msg/RequestBandIBCRates response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBandIbcRatesResponse {
}
/// MsgRelayPythPrices defines a SDK message for updating Pyth prices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPythPrices {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub price_attestations: ::prost::alloc::vec::Vec<PriceAttestation>,
}
/// MsgRelayPythPricesResponse defines the Msg/RelayPythPrices response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPythPricesResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the oracle parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
// @@protoc_insertion_point(module)
