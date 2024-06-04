// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// auction_period_duration defines the auction period duration
    #[prost(int64, tag="1")]
    pub auction_period: i64,
    /// min_next_bid_increment_rate defines the minimum increment rate for new bids
    #[prost(string, tag="2")]
    pub min_next_bid_increment_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBid {
    /// bidder describes the address of bidder
    #[prost(string, tag="1")]
    pub bidder: ::prost::alloc::string::String,
    /// amount describes the amount the bidder put on the auction
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag="1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionStart {
    /// round defines the round number of auction
    #[prost(uint64, tag="1")]
    pub round: u64,
    /// ending_timestamp describes auction end time
    #[prost(int64, tag="2")]
    pub ending_timestamp: i64,
    /// new_basket describes auction module balance at the time of new auction
    /// start
    #[prost(message, repeated, tag="3")]
    pub new_basket: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Bid defines a SDK message for placing a bid for an auction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// amount of the bid in INJ tokens
    #[prost(message, optional, tag="2")]
    pub bid_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// the current auction round being bid on
    #[prost(uint64, tag="3")]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the ocr parameters to update.
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
