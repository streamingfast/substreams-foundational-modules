// @generated
/// BridgeValidator represents a validator's ETH address and its power
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BridgeValidator {
    #[prost(uint64, tag="1")]
    pub power: u64,
    #[prost(string, tag="2")]
    pub ethereum_address: ::prost::alloc::string::String,
}
/// Valset is the Ethereum Bridge Multsig Set, each peggy validator also
/// maintains an ETH key to sign messages, these are used to check signatures on
/// ETH because of the significant gas savings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Valset {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(string, tag="4")]
    pub reward_amount: ::prost::alloc::string::String,
    /// the reward token in it's Ethereum hex address representation
    #[prost(string, tag="5")]
    pub reward_token: ::prost::alloc::string::String,
}
/// LastObservedEthereumBlockHeight stores the last observed
/// Ethereum block height along with the Cosmos block height that
/// it was observed at. These two numbers can be used to project
/// outward and always produce batches with timeouts in the future
/// even if no Ethereum block height has been relayed for a long time
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastObservedEthereumBlockHeight {
    #[prost(uint64, tag="1")]
    pub cosmos_block_height: u64,
    #[prost(uint64, tag="2")]
    pub ethereum_block_height: u64,
}
/// LastClaimEvent stores last claim event details of validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastClaimEvent {
    #[prost(uint64, tag="1")]
    pub ethereum_event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub ethereum_event_height: u64,
}
/// This records the relationship between an ERC20 token and the denom
/// of the corresponding Cosmos originated asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20ToDenom {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
// Params represent the peggy genesis and store parameters
// peggy_id:
// a random 32 byte value to prevent signature reuse, for example if the
// cosmos validators decided to use the same Ethereum keys for another chain
// also running Peggy we would not want it to be possible to play a deposit
// from chain A back on chain B's peggy. This value IS USED ON ETHEREUM so
// it must be set in your genesis.json before launch and not changed after
// deploying Peggy
//
// contract_hash:
// the code hash of a known good version of the Peggy contract
// solidity code. This can be used to verify the correct version
// of the contract has been deployed. This is a reference value for
// goernance action only it is never read by any Peggy code
//
// bridge_ethereum_address:
// is address of the bridge contract on the Ethereum side, this is a
// reference value for governance only and is not actually used by any
// Peggy code
//
// bridge_chain_id:
// the unique identifier of the Ethereum chain, this is a reference value
// only and is not actually used by any Peggy code
//
// These reference values may be used by future Peggy client implemetnations
// to allow for saftey features or convenience features like the peggy address
// in your relayer. A relayer would require a configured peggy address if
// governance had not set the address on the chain it was relaying for.
//
// signed_valsets_window
// signed_batches_window
// signed_claims_window
//
// These values represent the time in blocks that a validator has to submit
// a signature for a batch or valset, or to submit a claim for a particular
// attestation nonce. In the case of attestations this clock starts when the
// attestation is created, but only allows for slashing once the event has
// passed
//
// target_batch_timeout:
//
// This is the 'target' value for when batches time out, this is a target
// becuase Ethereum is a probabalistic chain and you can't say for sure what the
// block frequency is ahead of time.
//
// average_block_time
// average_ethereum_block_time
//
// These values are the average Cosmos block time and Ethereum block time
// repsectively and they are used to copute what the target batch timeout is. It
// is important that governance updates these in case of any major, prolonged
// change in the time it takes to produce a block
//
// slash_fraction_valset
// slash_fraction_batch
// slash_fraction_claim
// slash_fraction_conflicting_claim
//
// The slashing fractions for the various peggy related slashing conditions. The
// first three refer to not submitting a particular message, the third for
// submitting a different claim for the same Ethereum event
//
// unbond_slashing_valsets_window
//
// The unbond slashing valsets window is used to determine how many blocks after
// starting to unbond a validator needs to continue signing blocks. The goal of
// this paramater is that when a validator leaves the set, if their leaving
// creates enough change in the validator set to justify an update they will
// sign a validator set update for the Ethereum bridge that does not include
// themselves. Allowing us to remove them from the Ethereum bridge and replace
// them with the new set gracefully.
//
// valset_reward
//
// Valset rewards are the amount of tokens this chain issues to relayers of
// validator sets. These can be any ERC20 token in the bridge, but it's strongly
// advised that chains use only Cosmos originated tokens, which the bridge
// effectively mints on Ethereum. If you run out of the token you are using for
// validator set rewards valset updates will fail and the bridge will be
// vulnerable to highjacking. For these paramaters the zero values are special
// and indicate not to attempt any reward. This is the default for
// bootstrapping.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub peggy_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contract_source_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bridge_ethereum_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag="5")]
    pub signed_valsets_window: u64,
    #[prost(uint64, tag="6")]
    pub signed_batches_window: u64,
    #[prost(uint64, tag="7")]
    pub signed_claims_window: u64,
    #[prost(uint64, tag="8")]
    pub target_batch_timeout: u64,
    #[prost(uint64, tag="9")]
    pub average_block_time: u64,
    #[prost(uint64, tag="10")]
    pub average_ethereum_block_time: u64,
    #[prost(bytes="vec", tag="11")]
    pub slash_fraction_valset: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub slash_fraction_batch: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub slash_fraction_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub slash_fraction_conflicting_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="15")]
    pub unbond_slashing_valsets_window: u64,
    #[prost(bytes="vec", tag="16")]
    pub slash_fraction_bad_eth_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="17")]
    pub cosmos_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub cosmos_coin_erc20_contract: ::prost::alloc::string::String,
    #[prost(bool, tag="19")]
    pub claim_slashing_enabled: bool,
    #[prost(uint64, tag="20")]
    pub bridge_contract_start_height: u64,
    #[prost(message, optional, tag="21")]
    pub valset_reward: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetOrchestratorAddresses
/// this message allows validators to delegate their voting responsibilities
/// to a given key. This key is then used as an optional authentication method
/// for sigining oracle claims
/// VALIDATOR
/// The validator field is a cosmosvaloper1... string (i.e. sdk.ValAddress)
/// that references a validator in the active set
/// ORCHESTRATOR
/// The orchestrator field is a cosmos1... string  (i.e. sdk.AccAddress) that
/// references the key that is being delegated to
/// ETH_ADDRESS
/// This is a hex encoded 0x Ethereum public key that will be used by this
/// validator on Ethereum
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddresses {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddressesResponse {
}
/// MsgValsetConfirm
/// this is the message sent by the validators when they wish to submit their
/// signatures over the validator set at a given block height. A validator must
/// first call MsgSetEthAddress to set their Ethereum address to be used for
/// signing. Then someone (anyone) must make a ValsetRequest the request is
/// essentially a messaging mechanism to determine which block all validators
/// should submit signatures over. Finally validators sign the validator set,
/// powers, and Ethereum addresses of the entire validator set at the height of a
/// ValsetRequest and submit that signature with this message.
///
/// If a sufficient number of validators (66% of voting power) (A) have set
/// Ethereum addresses and (B) submit ValsetConfirm messages with their
/// signatures it is then possible for anyone to view these signatures in the
/// chain store and submit them to Ethereum to update the validator set
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirm {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirmResponse {
}
/// MsgSendToEth
/// This is the message that a user calls when they want to bridge an asset
/// it will later be removed when it is included in a batch and successfully
/// submitted tokens are removed from the users balance immediately
/// -------------
/// AMOUNT:
/// the coin to send across the bridge, note the restriction that this is a
/// single coin not a set of coins that is normal in other Cosmos messages
/// FEE:
/// the fee paid for the bridge, distinct from the fee paid to the chain to
/// actually send this message in the first place. So a successful send has
/// two layers of fees for the user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEth {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub eth_dest: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub bridge_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEthResponse {
}
/// MsgRequestBatch
/// this is a message anyone can send that requests a batch of transactions to
/// send across the bridge be created for whatever block height this message is
/// included in. This acts as a coordination point, the handler for this message
/// looks at the AddToOutgoingPool tx's in the store and generates a batch, also
/// available in the store tied to this message. The validators then grab this
/// batch, sign it, submit the signatures with a MsgConfirmBatch before a relayer
/// can finally submit the batch
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatch {
    #[prost(string, tag="1")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatchResponse {
}
/// MsgConfirmBatch
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (TODO determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatch {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatchResponse {
}
/// EthereumBridgeDepositClaim
/// When more than 66% of the active validator set has
/// claimed to have seen the deposit enter the ethereum blockchain coins are
/// issued to the Cosmos address in question
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaimResponse {
}
/// WithdrawClaim claims that a batch of withdrawal
/// operations on the bridge contract was executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(uint64, tag="3")]
    pub batch_nonce: u64,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaimResponse {
}
/// ERC20DeployedClaim allows the Cosmos module
/// to learn about an ERC20 that someone deployed
/// to represent a Cosmos asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
    #[prost(string, tag="8")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaimResponse {
}
/// This call allows the sender (and only the sender)
/// to cancel a given MsgSendToEth and recieve a refund
/// of the tokens
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEth {
    #[prost(uint64, tag="1")]
    pub transaction_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEthResponse {
}
/// This call allows anyone to submit evidence that a
/// validator has signed a valset, batch, or logic call that never
/// existed. Subject contains the batch, valset, or logic call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidence {
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidenceResponse {
}
/// This informs the Cosmos module that a validator
/// set has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub valset_nonce: u64,
    #[prost(uint64, tag="3")]
    pub block_height: u64,
    #[prost(message, repeated, tag="4")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag="5")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaimResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the peggy parameters to update.
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
