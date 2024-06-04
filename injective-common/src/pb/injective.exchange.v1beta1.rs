// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// spot_market_instant_listing_fee defines the expedited fee in INJ required
    /// to create a spot market by bypassing governance
    #[prost(message, optional, tag="1")]
    pub spot_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// derivative_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag="2")]
    pub derivative_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// default_spot_maker_fee defines the default exchange trade fee for makers on
    /// a spot market
    #[prost(string, tag="3")]
    pub default_spot_maker_fee_rate: ::prost::alloc::string::String,
    /// default_spot_taker_fee_rate defines the default exchange trade fee rate for
    /// takers on a new spot market
    #[prost(string, tag="4")]
    pub default_spot_taker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_maker_fee defines the default exchange trade fee for
    /// makers on a new derivative market
    #[prost(string, tag="5")]
    pub default_derivative_maker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_taker_fee defines the default exchange trade fee for
    /// takers on a new derivative market
    #[prost(string, tag="6")]
    pub default_derivative_taker_fee_rate: ::prost::alloc::string::String,
    /// default_initial_margin_ratio defines the default initial margin ratio on a
    /// new derivative market
    #[prost(string, tag="7")]
    pub default_initial_margin_ratio: ::prost::alloc::string::String,
    /// default_maintenance_margin_ratio defines the default maintenance margin
    /// ratio on a new derivative market
    #[prost(string, tag="8")]
    pub default_maintenance_margin_ratio: ::prost::alloc::string::String,
    /// default_funding_interval defines the default funding interval on a
    /// derivative market
    #[prost(int64, tag="9")]
    pub default_funding_interval: i64,
    /// funding_multiple defines the timestamp multiple that the funding timestamp
    /// should be a multiple of
    #[prost(int64, tag="10")]
    pub funding_multiple: i64,
    /// relayer_fee_share_rate defines the trade fee share percentage that goes to
    /// relayers
    #[prost(string, tag="11")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// default_hourly_funding_rate_cap defines the default maximum absolute value
    /// of the hourly funding rate
    #[prost(string, tag="12")]
    pub default_hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag="13")]
    pub default_hourly_interest_rate: ::prost::alloc::string::String,
    /// max_derivative_order_side_count defines the maximum number of derivative
    /// active orders a subaccount can have for a given orderbook side
    #[prost(uint32, tag="14")]
    pub max_derivative_order_side_count: u32,
    /// inj_reward_staked_requirement_threshold defines the threshold on INJ
    /// rewards after which one also needs staked INJ to receive more
    #[prost(string, tag="15")]
    pub inj_reward_staked_requirement_threshold: ::prost::alloc::string::String,
    /// the trading_rewards_vesting_duration defines the vesting times for trading
    /// rewards
    #[prost(int64, tag="16")]
    pub trading_rewards_vesting_duration: i64,
    /// liquidator_reward_share_rate defines the ratio of the split of the surplus
    /// collateral that goes to the liquidator
    #[prost(string, tag="17")]
    pub liquidator_reward_share_rate: ::prost::alloc::string::String,
    /// binary_options_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag="18")]
    pub binary_options_market_instant_listing_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// atomic_market_order_access_level defines the required access permissions
    /// for executing atomic market orders
    #[prost(enumeration="AtomicMarketOrderAccessLevel", tag="19")]
    pub atomic_market_order_access_level: i32,
    /// spot_atomic_market_order_fee_multiplier defines the default multiplier for
    /// executing atomic market orders in spot markets
    #[prost(string, tag="20")]
    pub spot_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// derivative_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in derivative markets
    #[prost(string, tag="21")]
    pub derivative_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// binary_options_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in binary markets
    #[prost(string, tag="22")]
    pub binary_options_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// minimal_protocol_fee_rate defines the minimal protocol fee rate
    #[prost(string, tag="23")]
    pub minimal_protocol_fee_rate: ::prost::alloc::string::String,
    /// is_instant_derivative_market_launch_enabled defines whether instant
    /// derivative market launch is enabled
    #[prost(bool, tag="24")]
    pub is_instant_derivative_market_launch_enabled: bool,
    #[prost(int64, tag="25")]
    pub post_only_mode_height_threshold: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFeeMultiplier {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fee_multiplier: ::prost::alloc::string::String,
}
/// An object describing a derivative market in the Injective Futures Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="2")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="3")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="5")]
    pub oracle_scale_factor: u32,
    /// Address of the quote currency denomination for the derivative contract
    #[prost(string, tag="6")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio of a derivative
    /// market
    #[prost(string, tag="8")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio of a
    /// derivative market
    #[prost(string, tag="9")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a derivative market
    #[prost(string, tag="10")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag="11")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="12")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// true if the market is a perpetual market. false if the market is an expiry
    /// futures market
    #[prost(bool, tag="13")]
    pub is_perpetual: bool,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// An object describing a binary options market in Injective Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="2")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag="3")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="5")]
    pub oracle_scale_factor: u32,
    /// expiration timestamp
    #[prost(int64, tag="6")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="7")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag="8")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag="9")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="10")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a binary options market
    #[prost(string, tag="11")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag="12")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="13")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// expiration_timestamp defines the expiration time for a time expiry futures
    /// market.
    #[prost(int64, tag="2")]
    pub expiration_timestamp: i64,
    /// expiration_twap_start_timestamp defines the start time of the TWAP
    /// calculation window
    #[prost(int64, tag="3")]
    pub twap_start_timestamp: i64,
    /// expiration_twap_start_price_cumulative defines the cumulative price for the
    /// start of the TWAP window
    #[prost(string, tag="4")]
    pub expiration_twap_start_price_cumulative: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price for a time expiry futures
    /// market.
    #[prost(string, tag="5")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// hourly_funding_rate_cap defines the maximum absolute value of the hourly
    /// funding rate
    #[prost(string, tag="2")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag="3")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// next_funding_timestamp defines the next funding timestamp in seconds of a
    /// perpetual market
    #[prost(int64, tag="4")]
    pub next_funding_timestamp: i64,
    /// funding_interval defines the next funding interval in seconds of a
    /// perpetual market.
    #[prost(int64, tag="5")]
    pub funding_interval: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFunding {
    /// cumulative_funding defines the cumulative funding of a perpetual market.
    #[prost(string, tag="1")]
    pub cumulative_funding: ::prost::alloc::string::String,
    /// cumulative_price defines the cumulative price for the current hour up to
    /// the last timestamp
    #[prost(string, tag="2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub last_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketSettlementInfo {
    /// market ID.
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price
    #[prost(string, tag="2")]
    pub settlement_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextFundingTimestamp {
    #[prost(int64, tag="1")]
    pub next_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidPriceAndTob {
    /// mid price of the market
    #[prost(string, tag="1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag="2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag="3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
/// An object describing trade pair of two assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarket {
    /// A name of the pair in format AAA/BBB, where AAA is base asset, BBB is quote
    /// asset.
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom used for the base asset
    #[prost(string, tag="2")]
    pub base_denom: ::prost::alloc::string::String,
    /// Coin used for the quote asset
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// maker_fee_rate defines the fee percentage makers pay when trading
    #[prost(string, tag="4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the fee percentage takers pay when trading
    #[prost(string, tag="5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag="6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="8")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price required
    /// for orders in the market
    #[prost(string, tag="9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// A subaccount's deposit for a given base currency
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(string, tag="1")]
    pub available_balance: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub total_balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountTradeNonce {
    #[prost(uint32, tag="1")]
    pub nonce: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderInfo {
    /// bytes32 subaccount ID that created the order
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// address fee_recipient address that will receive fees for the order
    #[prost(string, tag="2")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// price of the order
    #[prost(string, tag="3")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag="2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="3")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="4")]
    pub trigger_price: ::prost::alloc::string::String,
}
/// A valid Spot limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="3")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="4")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Spot market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    #[prost(string, tag="2")]
    pub balance_hold: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    /// order types
    #[prost(enumeration="OrderType", tag="4")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag="2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="3")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag="4")]
    pub margin: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderbookMetadata {
    #[prost(uint32, tag="1")]
    pub vanilla_limit_order_count: u32,
    #[prost(uint32, tag="2")]
    pub reduce_only_limit_order_count: u32,
    /// AggregateReduceOnlyQuantity is the aggregate fillable quantity of the
    /// subaccount's reduce-only limit orders in the given direction.
    #[prost(string, tag="3")]
    pub aggregate_reduce_only_quantity: ::prost::alloc::string::String,
    /// AggregateVanillaQuantity is the aggregate fillable quantity of the
    /// subaccount's vanilla limit orders in the given direction.
    #[prost(string, tag="4")]
    pub aggregate_vanilla_quantity: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub vanilla_conditional_order_count: u32,
    #[prost(uint32, tag="6")]
    pub reduce_only_conditional_order_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrder {
    /// price of the order
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_reduce_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderData {
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<SubaccountOrder>,
    #[prost(bytes="vec", tag="2")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Derivative limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag="4")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A valid Derivative market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag="1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration="OrderType", tag="2")]
    pub order_type: i32,
    #[prost(string, tag="3")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub margin_hold: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag="5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(bool, tag="1")]
    pub is_long: bool,
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cumulative_funding_entry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderIndicator {
    /// market_id represents the unique ID of the market
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLog {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    /// bytes32 subaccount ID that executed the trade
    #[prost(bytes="vec", tag="3")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDelta {
    #[prost(bool, tag="1")]
    pub is_long: bool,
    #[prost(string, tag="2")]
    pub execution_quantity: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub execution_margin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub execution_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTradeLog {
    #[prost(bytes="vec", tag="1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag="3")]
    pub payout: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountPosition {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    #[prost(bytes="vec", tag="2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(bytes="vec", tag="1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub deposit: ::core::option::Option<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositUpdate {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub deposits: ::prost::alloc::vec::Vec<SubaccountDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsMultiplier {
    #[prost(string, tag="1")]
    pub maker_points_multiplier: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_points_multiplier: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignBoostInfo {
    #[prost(string, repeated, tag="1")]
    pub boosted_spot_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub spot_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
    #[prost(string, repeated, tag="3")]
    pub boosted_derivative_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub derivative_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignRewardPool {
    #[prost(int64, tag="1")]
    pub start_timestamp: i64,
    /// max_campaign_rewards are the maximum reward amounts to be disbursed at the
    /// end of the campaign
    #[prost(message, repeated, tag="2")]
    pub max_campaign_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignInfo {
    /// number of seconds of the duration of each campaign
    #[prost(int64, tag="1")]
    pub campaign_duration_seconds: i64,
    /// the trading fee quote denoms which will be counted for the rewards
    #[prost(string, repeated, tag="2")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the optional boost info for markets
    #[prost(message, optional, tag="3")]
    pub trading_reward_boost_info: ::core::option::Option<TradingRewardCampaignBoostInfo>,
    /// the marketIDs which are disqualified from being rewarded
    #[prost(string, repeated, tag="4")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierInfo {
    #[prost(string, tag="1")]
    pub maker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub staked_amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountSchedule {
    #[prost(uint64, tag="1")]
    pub bucket_count: u64,
    #[prost(int64, tag="2")]
    pub bucket_duration: i64,
    /// the trading fee quote denoms which will be counted for the fee paid
    /// contribution
    #[prost(string, repeated, tag="3")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the fee discount tiers
    #[prost(message, repeated, tag="4")]
    pub tier_infos: ::prost::alloc::vec::Vec<FeeDiscountTierInfo>,
    /// the marketIDs which are disqualified from contributing to the fee paid
    /// amount
    #[prost(string, repeated, tag="5")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierTtl {
    #[prost(uint64, tag="1")]
    pub tier: u64,
    #[prost(int64, tag="2")]
    pub ttl_timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeRecord {
    #[prost(string, tag="1")]
    pub maker_volume: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub taker_volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRewards {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecords {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub latest_trade_records: ::prost::alloc::vec::Vec<TradeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountIDs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecord {
    #[prost(int64, tag="1")]
    pub timestamp: i64,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// price
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    /// quantity
    #[prost(string, tag="2")]
    pub q: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateSubaccountVolumeRecord {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateAccountVolumeRecord {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketVolume {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub volume: ::core::option::Option<VolumeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomDecimals {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub decimals: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtomicMarketOrderAccessLevel {
    Nobody = 0,
    /// currently unsupported
    BeginBlockerSmartContractsOnly = 1,
    SmartContractsOnly = 2,
    Everyone = 3,
}
impl AtomicMarketOrderAccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AtomicMarketOrderAccessLevel::Nobody => "Nobody",
            AtomicMarketOrderAccessLevel::BeginBlockerSmartContractsOnly => "BeginBlockerSmartContractsOnly",
            AtomicMarketOrderAccessLevel::SmartContractsOnly => "SmartContractsOnly",
            AtomicMarketOrderAccessLevel::Everyone => "Everyone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Nobody" => Some(Self::Nobody),
            "BeginBlockerSmartContractsOnly" => Some(Self::BeginBlockerSmartContractsOnly),
            "SmartContractsOnly" => Some(Self::SmartContractsOnly),
            "Everyone" => Some(Self::Everyone),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketStatus {
    Unspecified = 0,
    Active = 1,
    Paused = 2,
    Demolished = 3,
    Expired = 4,
}
impl MarketStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketStatus::Unspecified => "Unspecified",
            MarketStatus::Active => "Active",
            MarketStatus::Paused => "Paused",
            MarketStatus::Demolished => "Demolished",
            MarketStatus::Expired => "Expired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Active" => Some(Self::Active),
            "Paused" => Some(Self::Paused),
            "Demolished" => Some(Self::Demolished),
            "Expired" => Some(Self::Expired),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Unspecified = 0,
    Buy = 1,
    Sell = 2,
    StopBuy = 3,
    StopSell = 4,
    TakeBuy = 5,
    TakeSell = 6,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "UNSPECIFIED",
            OrderType::Buy => "BUY",
            OrderType::Sell => "SELL",
            OrderType::StopBuy => "STOP_BUY",
            OrderType::StopSell => "STOP_SELL",
            OrderType::TakeBuy => "TAKE_BUY",
            OrderType::TakeSell => "TAKE_SELL",
            OrderType::BuyPo => "BUY_PO",
            OrderType::SellPo => "SELL_PO",
            OrderType::BuyAtomic => "BUY_ATOMIC",
            OrderType::SellAtomic => "SELL_ATOMIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            "STOP_BUY" => Some(Self::StopBuy),
            "STOP_SELL" => Some(Self::StopSell),
            "TAKE_BUY" => Some(Self::TakeBuy),
            "TAKE_SELL" => Some(Self::TakeSell),
            "BUY_PO" => Some(Self::BuyPo),
            "SELL_PO" => Some(Self::SellPo),
            "BUY_ATOMIC" => Some(Self::BuyAtomic),
            "SELL_ATOMIC" => Some(Self::SellAtomic),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    UnspecifiedExecutionType = 0,
    Market = 1,
    LimitFill = 2,
    LimitMatchRestingOrder = 3,
    LimitMatchNewOrder = 4,
    MarketLiquidation = 5,
    ExpiryMarketSettlement = 6,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::UnspecifiedExecutionType => "UnspecifiedExecutionType",
            ExecutionType::Market => "Market",
            ExecutionType::LimitFill => "LimitFill",
            ExecutionType::LimitMatchRestingOrder => "LimitMatchRestingOrder",
            ExecutionType::LimitMatchNewOrder => "LimitMatchNewOrder",
            ExecutionType::MarketLiquidation => "MarketLiquidation",
            ExecutionType::ExpiryMarketSettlement => "ExpiryMarketSettlement",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnspecifiedExecutionType" => Some(Self::UnspecifiedExecutionType),
            "Market" => Some(Self::Market),
            "LimitFill" => Some(Self::LimitFill),
            "LimitMatchRestingOrder" => Some(Self::LimitMatchRestingOrder),
            "LimitMatchNewOrder" => Some(Self::LimitMatchNewOrder),
            "MarketLiquidation" => Some(Self::MarketLiquidation),
            "ExpiryMarketSettlement" => Some(Self::ExpiryMarketSettlement),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderMask {
    Unused = 0,
    Any = 1,
    Regular = 2,
    Conditional = 4,
    /// for conditional orders means HIGHER
    DirectionBuyOrHigher = 8,
    /// for conditional orders means LOWER
    DirectionSellOrLower = 16,
    TypeMarket = 32,
    TypeLimit = 64,
}
impl OrderMask {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderMask::Unused => "UNUSED",
            OrderMask::Any => "ANY",
            OrderMask::Regular => "REGULAR",
            OrderMask::Conditional => "CONDITIONAL",
            OrderMask::DirectionBuyOrHigher => "DIRECTION_BUY_OR_HIGHER",
            OrderMask::DirectionSellOrLower => "DIRECTION_SELL_OR_LOWER",
            OrderMask::TypeMarket => "TYPE_MARKET",
            OrderMask::TypeLimit => "TYPE_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNUSED" => Some(Self::Unused),
            "ANY" => Some(Self::Any),
            "REGULAR" => Some(Self::Regular),
            "CONDITIONAL" => Some(Self::Conditional),
            "DIRECTION_BUY_OR_HIGHER" => Some(Self::DirectionBuyOrHigher),
            "DIRECTION_SELL_OR_LOWER" => Some(Self::DirectionSellOrLower),
            "TYPE_MARKET" => Some(Self::TypeMarket),
            "TYPE_LIMIT" => Some(Self::TypeLimit),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the exchange parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// MsgDeposit defines a SDK message for transferring coins from the sender's
/// bank balance into the subaccount's exchange deposits
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// (Optional) bytes32 subaccount ID to deposit funds into. If empty, the coin
    /// will be deposited to the sender's default subaccount address.
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositResponse defines the Msg/Deposit response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {
}
/// MsgWithdraw defines a SDK message for withdrawing coins from a subaccount's
/// deposits to the user's bank balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// bytes32 subaccount ID to withdraw funds from
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdraw defines the Msg/Withdraw response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {
}
/// MsgCreateSpotLimitOrder defines a SDK message for creating a new spot limit
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<SpotOrder>,
}
/// MsgCreateSpotLimitOrderResponse defines the Msg/CreateSpotOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// MsgBatchCreateSpotLimitOrders defines a SDK message for creating a new batch
/// of spot limit orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<SpotOrder>,
}
/// MsgBatchCreateSpotLimitOrdersResponse defines the
/// Msg/BatchCreateSpotLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrdersResponse {
    #[prost(string, repeated, tag="1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgInstantSpotMarketLaunch defines a SDK message for creating a new spot
/// market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the spot market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="3")]
    pub base_denom: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price
    #[prost(string, tag="5")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="6")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantSpotMarketLaunchResponse defines the Msg/InstantSpotMarketLaunch
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunchResponse {
}
/// MsgInstantPerpetualMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="6")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="7")]
    pub oracle_type: i32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag="8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag="9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the perpetual
    /// market
    #[prost(string, tag="10")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// perpetual market
    #[prost(string, tag="11")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="12")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="13")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantPerpetualMarketLaunchResponse defines the
/// Msg/InstantPerpetualMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunchResponse {
}
/// MsgInstantBinaryOptionsMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative contract.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag="3")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag="4")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="5")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="6")]
    pub oracle_scale_factor: u32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag="7")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag="8")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag="9")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="10")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag="11")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag="12")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag="13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag="14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantBinaryOptionsMarketLaunchResponse defines the
/// Msg/InstantBinaryOptionsMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunchResponse {
}
/// MsgInstantExpiryFuturesMarketLaunch defines a SDK message for creating a new
/// expiry futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag="3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag="4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag="5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration="super::super::oracle::v1beta1::OracleType", tag="6")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag="7")]
    pub oracle_scale_factor: u32,
    /// Expiration time of the market
    #[prost(int64, tag="8")]
    pub expiry: i64,
    /// maker_fee_rate defines the trade fee rate for makers on the expiry futures
    /// market
    #[prost(string, tag="9")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the expiry futures
    /// market
    #[prost(string, tag="10")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag="11")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag="12")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag="13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag="14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}
/// MsgInstantExpiryFuturesMarketLaunchResponse defines the
/// Msg/InstantExpiryFuturesMarketLaunch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunchResponse {
}
/// MsgCreateSpotMarketOrder defines a SDK message for creating a new spot market
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<SpotOrder>,
}
/// MsgCreateSpotMarketOrderResponse defines the Msg/CreateSpotMarketLimitOrder
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<SpotMarketOrderResults>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrderResults {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub fee: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateDerivativeLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateDerivativeLimitOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateBinaryOptionsLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateBinaryOptionsLimitOrderResponse defines the
/// Msg/CreateBinaryOptionsLimitOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgBatchCreateDerivativeLimitOrders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
/// MsgBatchCreateDerivativeLimitOrdersResponse defines the
/// Msg/BatchCreateDerivativeLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrdersResponse {
    #[prost(string, repeated, tag="1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCancelSpotOrder defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cid: ::prost::alloc::string::String,
}
/// MsgCancelSpotOrderResponse defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrderResponse {
}
/// MsgBatchCancelSpotOrders defines the Msg/BatchCancelSpotOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// MsgBatchCancelSpotOrdersResponse defines the Msg/BatchCancelSpotOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// MsgBatchCancelBinaryOptionsOrders defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// BatchCancelBinaryOptionsOrdersResponse defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// MsgBatchUpdateOrders defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// subaccount_id only used for the spot_market_ids_to_cancel_all and
    /// derivative_market_ids_to_cancel_all.
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub spot_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub derivative_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub spot_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag="6")]
    pub derivative_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag="7")]
    pub spot_orders_to_create: ::prost::alloc::vec::Vec<SpotOrder>,
    #[prost(message, repeated, tag="8")]
    pub derivative_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
    #[prost(message, repeated, tag="9")]
    pub binary_options_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(string, repeated, tag="10")]
    pub binary_options_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="11")]
    pub binary_options_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
/// MsgBatchUpdateOrdersResponse defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub spot_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(bool, repeated, tag="2")]
    pub derivative_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="3")]
    pub spot_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub derivative_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, repeated, tag="5")]
    pub binary_options_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag="6")]
    pub binary_options_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Cosmos-SDK MsgCreateDerivativeMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateDerivativeMarketOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrderResults {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag="5")]
    pub payout: ::prost::alloc::string::String,
}
/// A Cosmos-SDK MsgCreateBinaryOptionsMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgCreateBinaryOptionsMarketOrderResponse defines the
/// Msg/CreateBinaryOptionsMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrderResponse {
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
}
/// MsgCancelDerivativeOrder defines the Msg/CancelDerivativeOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="5")]
    pub order_mask: i32,
    #[prost(string, tag="6")]
    pub cid: ::prost::alloc::string::String,
}
/// MsgCancelDerivativeOrderResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrderResponse {
}
/// MsgCancelBinaryOptionsOrder defines the Msg/CancelBinaryOptionsOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrder {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="5")]
    pub order_mask: i32,
    #[prost(string, tag="6")]
    pub cid: ::prost::alloc::string::String,
}
/// MsgCancelBinaryOptionsOrderResponse defines the
/// Msg/CancelBinaryOptionsOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrderResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderData {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag="4")]
    pub order_mask: i32,
    #[prost(string, tag="5")]
    pub cid: ::prost::alloc::string::String,
}
/// MsgBatchCancelDerivativeOrders defines the Msg/CancelDerivativeOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrders {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
/// MsgBatchCancelDerivativeOrdersResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrdersResponse {
    #[prost(bool, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
/// A Cosmos-SDK MsgSubaccountTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSubaccountTransferResponse defines the Msg/SubaccountTransfer response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransferResponse {
}
/// A Cosmos-SDK MsgExternalTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgExternalTransferResponse defines the Msg/ExternalTransfer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransferResponse {
}
/// A Cosmos-SDK MsgLiquidatePosition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePosition {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// optional order to provide for liquidation
    #[prost(message, optional, tag="4")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
/// MsgLiquidatePositionResponse defines the Msg/LiquidatePosition response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePositionResponse {
}
/// A Cosmos-SDK MsgEmergencySettleMarket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmergencySettleMarket {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
}
/// MsgEmergencySettleMarketResponse defines the Msg/EmergencySettleMarket
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmergencySettleMarketResponse {
}
/// A Cosmos-SDK MsgIncreasePositionMargin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMargin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
    /// amount defines the amount of margin to add to the position
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgIncreasePositionMarginResponse defines the Msg/IncreasePositionMargin
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMarginResponse {
}
/// MsgPrivilegedExecuteContract defines the Msg/Exec message type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContract {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// funds defines the user's bank coins used to fund the execution (e.g.
    /// 100inj).
    #[prost(string, tag="2")]
    pub funds: ::prost::alloc::string::String,
    /// contract_address defines the contract address to execute
    #[prost(string, tag="3")]
    pub contract_address: ::prost::alloc::string::String,
    /// data defines the call data used when executing the contract
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
}
/// MsgPrivilegedExecuteContractResponse defines the Msg/Exec response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContractResponse {
    #[prost(message, repeated, tag="1")]
    pub funds_diff: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// A Cosmos-SDK MsgRewardsOptOut
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOut {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgRewardsOptOutResponse defines the Msg/RewardsOptOut response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOutResponse {
}
/// A Cosmos-SDK MsgReclaimLockedFunds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFunds {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub locked_account_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// MsgReclaimLockedFundsResponse defines the Msg/ReclaimLockedFunds response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFundsResponse {
}
/// MsgSignData defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignData {
    /// Signer is the sdk.AccAddress of the message signer
    #[prost(bytes="vec", tag="1")]
    pub signer: ::prost::alloc::vec::Vec<u8>,
    /// Data represents the raw bytes of the content that is signed (text, json,
    /// etc)
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgSignDoc defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignDoc {
    #[prost(string, tag="1")]
    pub sign_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<MsgSignData>,
}
/// MsgAdminUpdateBinaryOptionsMarket is used by the market Admin to operate the
/// market
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarket {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// new price at which market will be settled
    #[prost(string, tag="3")]
    pub settlement_price: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag="4")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag="5")]
    pub settlement_timestamp: i64,
    /// Status of the market
    #[prost(enumeration="MarketStatus", tag="6")]
    pub status: i32,
}
/// MsgAdminUpdateBinaryOptionsMarketResponse is the response for
/// AdminUpdateBinaryOptionsMarket rpc method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarketResponse {
}
// @@protoc_insertion_point(module)
