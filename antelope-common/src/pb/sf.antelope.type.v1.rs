// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionTraces {
    #[prost(message, repeated, tag="1")]
    pub action_traces: ::prost::alloc::vec::Vec<ActionTrace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTraces {
    #[prost(message, repeated, tag="1")]
    pub transaction_traces: ::prost::alloc::vec::Vec<TransactionTrace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbOps {
    #[prost(message, repeated, tag="1")]
    pub db_ops: ::prost::alloc::vec::Vec<DbOp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// id is the block's hash.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Number is the block's height at which this block was mined.
    #[prost(uint32, tag="2")]
    pub number: u32,
    #[prost(uint32, tag="3")]
    pub version: u32,
    /// Header contain's the block's header information like its parent hash, the merkel root hash
    /// and all other information the form a block.
    #[prost(message, optional, tag="4")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(string, tag="5")]
    pub producer_signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub block_extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(uint32, tag="8")]
    pub dpos_proposed_irreversible_blocknum: u32,
    #[prost(uint32, tag="9")]
    pub dpos_irreversible_blocknum: u32,
    #[prost(message, optional, tag="11")]
    pub blockroot_merkle: ::core::option::Option<BlockRootMerkle>,
    #[prost(message, repeated, tag="12")]
    pub producer_to_last_produced: ::prost::alloc::vec::Vec<ProducerToLastProduced>,
    #[prost(message, repeated, tag="13")]
    pub producer_to_last_implied_irb: ::prost::alloc::vec::Vec<ProducerToLastImpliedIrb>,
    #[prost(uint32, repeated, tag="15")]
    pub confirm_count: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag="16")]
    pub pending_schedule: ::core::option::Option<PendingProducerSchedule>,
    #[prost(message, optional, tag="17")]
    pub activated_protocol_features: ::core::option::Option<ActivatedProtocolFeatures>,
    #[prost(message, repeated, tag="19")]
    pub rlimit_ops: ::prost::alloc::vec::Vec<RlimitOp>,
    /// The unfiltered transactions in this block when NO filtering has been applied,
    /// (i.e. `filtering_applied = false`). When filtering has been applied on this block,
    /// (i.e. `filtering_applied = true`), this field will be set to `nil` and instead, the
    /// `filtered_transactions` will be populated with only filtered transactions.
    ///
    /// Use the helper getter method `Transactions()` to automatically pick the correct
    /// field to use (`unfiltered_transactions` when `filtering_applied == false` and
    /// `filtered_transactions` when `filtering_applied == true`).
    #[prost(message, repeated, tag="6")]
    pub unfiltered_transactions: ::prost::alloc::vec::Vec<TransactionReceipt>,
    /// The filtered transactions in this block when filtering has been applied,
    /// (i.e. `filtering_applied = true`). This will be only the transactions
    /// that matched the include filter CEL expression and did NOT match the exclude
    /// filter CEL expression.
    ///
    /// Use the helper getter method `Transactions()` to automatically the correct
    /// field (`unfiltered_transaction` when `filtering_applied == false` and
    /// `filtered_transactions` when `filtering_applied == true`).
    #[prost(message, repeated, tag="47")]
    pub filtered_transactions: ::prost::alloc::vec::Vec<TransactionReceipt>,
    /// Number of transaction executed within this block when no filtering
    /// is applied (`filtering_applied == false`).
    #[prost(uint32, tag="22")]
    pub unfiltered_transaction_count: u32,
    /// Number of transaction that were successfully executed within this block that are found in
    /// the `filtered_transactions` array. This field is populated only when the flag
    /// `filtering_applied` is `true`.
    #[prost(uint32, tag="48")]
    pub filtered_transaction_count: u32,
    /// The unfiltered implicit transaction ops in this block when NO filtering has been applied,
    /// (i.e. `filtering_applied = false`). When filtering has been applied on this block,
    /// (i.e. `filtering_applied = true`), this field will be set to `nil` and instead, the
    /// `filtered_implicit_transaction_ops` will be populated with only filtered implicit
    /// transaction ops.
    ///
    /// Use the helper getter method `ImplicitTransactionOps()` to automatically pick the correct
    /// field to use (`unfiltered_implicit_transaction_ops` when `filtering_applied == false` and
    /// `filtered_implicit_transaction_ops` when `filtering_applied == true`).
    #[prost(message, repeated, tag="20")]
    pub unfiltered_implicit_transaction_ops: ::prost::alloc::vec::Vec<TrxOp>,
    /// The filtered implicit transaction ops in this block when filtering has been applied,
    /// (i.e. `filtering_applied = true`). This will be only the implicit transaction ops
    /// that matched the include filter CEL expression and did NOT match the exclude
    /// filter CEL expression.
    ///
    /// Use the helper getter method `ImplicitTransactionOps()` to automatically the correct
    /// field (`unfiltered_implicit_transaction_ops` when `filtering_applied == false` and
    /// `filtered_implicit_transaction_ops` when `filtering_applied == true`).
    #[prost(message, repeated, tag="49")]
    pub filtered_implicit_transaction_ops: ::prost::alloc::vec::Vec<TrxOp>,
    /// The unfiltered transaction traces in this block when NO filtering has been applied,
    /// (i.e. `filtering_applied = false`). When filtering has been applied on this block,
    /// (i.e. `filtering_applied = true`), this field will be set to `nil` and instead, the
    /// `filtered_transaction_traces` will be populated with only filtered transactions.
    ///
    /// Use the helper getter method `TransactionTraces()` to automatically pick the correct
    /// field to use (`unfiltered_transaction_traces` when `filtering_applied == false` and
    /// `filtered_transaction_traces` when `filtering_applied == true`).
    #[prost(message, repeated, tag="21")]
    pub unfiltered_transaction_traces: ::prost::alloc::vec::Vec<TransactionTrace>,
    /// The filtered transaction traces in this block when filtering has been applied,
    /// (i.e. `filtering_applied = true`). This will be only the transaction trace
    /// that matched the include filter CEL expression and did NOT match the exclude
    /// filter CEL expression.
    ///
    /// Use the helper getter method `TransactionTraces()` to automatically pick the correct
    /// field to use (`unfiltered_transaction_traces` when `filtering_applied == false` and
    /// `filtered_transaction_traces` when `filtering_applied == true`).
    #[prost(message, repeated, tag="46")]
    pub filtered_transaction_traces: ::prost::alloc::vec::Vec<TransactionTrace>,
    /// Number of transaction trace executed within this block when no filtering
    /// is applied (`filtering_applied == false`).
    #[prost(uint32, tag="23")]
    pub unfiltered_transaction_trace_count: u32,
    /// Number of transaction trace that were successfully executed within this block that are found in
    /// the `filtered_transaction_traces` array. This field is populated only when the flag
    /// `filtering_applied` is `true`.
    #[prost(uint32, tag="43")]
    pub filtered_transaction_trace_count: u32,
    /// Number of top-level actions that were successfully executed within this block when no filtering
    /// is applied (`filtering_applied == false`).
    #[prost(uint32, tag="24")]
    pub unfiltered_executed_input_action_count: u32,
    /// Number of top-level actions that were successfully executed within this block that are found in
    /// the `filtered_transaction_traces` array. This field is populated only when the flag
    /// `filtering_applied` is `true`.
    #[prost(uint32, tag="44")]
    pub filtered_executed_input_action_count: u32,
    /// Number of actions that were successfully executed within this block when no filtering
    /// is applied (`filtering_applied == false`).
    #[prost(uint32, tag="25")]
    pub unfiltered_executed_total_action_count: u32,
    /// Number of actions that were successfully executed within this block that are found in
    /// the `filtered_transaction_traces` array. This field is populated only when the flag
    /// `filtering_applied` is `true`.
    #[prost(uint32, tag="45")]
    pub filtered_executed_total_action_count: u32,
    // Reserved 26 to 29

    // / EOSIO 1.x only

    /// This was a single string element representing a public key (eos-go#ecc.PublicKey).
    /// It has been replaced by `valid_block_signing_authority_v2`.
    #[prost(string, tag="14")]
    pub block_signing_key: ::prost::alloc::string::String,
    /// This was a list of `{name, publicKey}` elements, each block being signed by a single key,
    /// the schedule was simply a list of pair, each pair being the producer name and it's public key
    /// used to sign the block.
    #[prost(message, optional, tag="10")]
    pub active_schedule_v1: ::core::option::Option<ProducerSchedule>,
    // / EOSIO 2.0.x only

    /// This replaces `block_signing_key` with a richer structure
    /// able to handle the weighted threshold multisig for block producers.
    ///
    /// This can be downgraded to the old `block_signing_key` simply by taking
    /// the first key present in the list. This is of course simple and not
    /// accurate anymore in EOSIO 2.0 system where `WTMSIG_BLOCK_SIGNATURES`
    /// has been activated AND block producers starts signing blocks with
    /// more than one key.
    ///
    /// See BlockSigningAuthority for further details
    #[prost(message, optional, tag="30")]
    pub valid_block_signing_authority_v2: ::core::option::Option<BlockSigningAuthority>,
    /// This repleaces the old type `ProducerSchedule` for the `active_schedule`
    /// field. This was only a type change in EOSIO 2.0, the field's name remained
    /// the same.
    ///
    /// This is the new schedule data layout which is richer than it's oldest
    /// counterpart. The inner element for a producer can then be composed with
    /// multiple keys, each with their own weight and the threshold required to
    /// accept the block signature.
    #[prost(message, optional, tag="31")]
    pub active_schedule_v2: ::core::option::Option<ProducerAuthoritySchedule>,
    /// Wheter or not a filtering process was run on this block. The filtering process sets to nil
    /// the `unfiltered_transaction_traces` to `nil` and populate the `filtered_transaction_traces`
    /// according to the `filtering_include_filter_expr` and `filtering_exclude_filter_expr` CEL
    /// expressions. A transaction will be present in the `filtered_transaction_traces` array if
    /// it matched the `filtering_include_filter_expr` and did *NOT* match the `filtering_exclude_filter_expr`.
    ///
    /// Moreover, each matching action that brought the transaction to be in `filtered_transaction_traces`
    /// array will have a `filtering_matched` flag set on it to broadcast the fact that this action
    /// match the inclusion/exclusion list.
    ///
    /// This flag controls all `filtered_*` and `unfiltered_*` elements on the Block structure and on
    /// substructures if present.
    #[prost(bool, tag="40")]
    pub filtering_applied: bool,
    /// The CEL filter expression used to include transaction in `filtered_transaction_traces` array, works
    /// in combination with `filtering_exclude_filter_expr` value.
    #[prost(string, tag="41")]
    pub filtering_include_filter_expr: ::prost::alloc::string::String,
    /// The CEL filter expression used to exclude transaction in `filtered_transaction_traces` array, works
    /// in combination with `filtering_include_filter_expr` value.
    #[prost(string, tag="42")]
    pub filtering_exclude_filter_expr: ::prost::alloc::string::String,
    /// The CEL filter expression used to include system actions, required by some systems, works
    /// in combination with the two other filters above.
    #[prost(string, tag="50")]
    pub filtering_system_actions_include_filter_expr: ::prost::alloc::string::String,
}
/// BlockWithRefs is a lightweight block, with traces and transactions
/// purged from the `block` within, and only.  It is used in transports
/// to pass block data around.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWithRefs {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub block: ::core::option::Option<Block>,
    /// TODO: Triple check that that's the right thing
    #[prost(message, optional, tag="3")]
    pub implicit_transaction_refs: ::core::option::Option<TransactionRefs>,
    #[prost(message, optional, tag="4")]
    pub transaction_refs: ::core::option::Option<TransactionRefs>,
    #[prost(message, optional, tag="5")]
    pub transaction_trace_refs: ::core::option::Option<TransactionRefs>,
    #[prost(bool, tag="6")]
    pub irreversible: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionRefs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivatedProtocolFeatures {
    #[prost(bytes="vec", repeated, tag="1")]
    pub protocol_features: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingProducerSchedule {
    #[prost(uint32, tag="1")]
    pub schedule_lib_num: u32,
    #[prost(bytes="vec", tag="2")]
    pub schedule_hash: ::prost::alloc::vec::Vec<u8>,
    // / EOSIO 1.x only

    /// See Block#active_schedule_v1 for further details, this is the same change
    /// as the active schedule, but applied to the pending field.
    #[prost(message, optional, tag="3")]
    pub schedule_v1: ::core::option::Option<ProducerSchedule>,
    // / EOSIO 2.0.x only

    /// See Block#active_schedule_v2 for further details, this is the same change
    /// as the active schedule, but applied to the pending field.
    #[prost(message, optional, tag="4")]
    pub schedule_v2: ::core::option::Option<ProducerAuthoritySchedule>,
}
/// Present in EOSIO 1.x only
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerSchedule {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(message, repeated, tag="2")]
    pub producers: ::prost::alloc::vec::Vec<ProducerKey>,
}
/// Present in EOSIO 1.x only
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerKey {
    #[prost(string, tag="1")]
    pub account_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub block_signing_key: ::prost::alloc::string::String,
}
/// Present in EOSIO 2.x only
///
/// This is the new schedule data layout which is richer than it's oldest
/// counterpart. The inner element for a producer can then be composed with
/// multiple keys, each with their own weight and the threshold required to
/// accept the block signature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerAuthoritySchedule {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(message, repeated, tag="2")]
    pub producers: ::prost::alloc::vec::Vec<ProducerAuthority>,
}
/// Present in EOSIO 2.x only
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerAuthority {
    #[prost(string, tag="1")]
    pub account_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub block_signing_authority: ::core::option::Option<BlockSigningAuthority>,
}
/// Present in EOSIO 2.x only
///
/// This represents the signatures that were used to signed the block. Previously,
/// in EOSIO 1.x, this was a simple public key since only one key could sign a block.
/// In EOSIO 2.x, when `WTMSIG_BLOCK_SIGNATURES` feature is active, the block can be
/// signed with a set of different public keys, each with its own weight as well as
/// the threshold at which point the signatures are accepted.
///
/// This is actually implemented as a `fc::variant` type in the C++ code, this tainted
/// our own implementation where multiple types can be represented using a `oneof`.
///
/// Know current types (and version they were introduced):
/// - `BlockSigningAuthorityV0` [Type 0] (EOSIO 2.0)
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSigningAuthority {
    #[prost(oneof="block_signing_authority::Variant", tags="1")]
    pub variant: ::core::option::Option<block_signing_authority::Variant>,
}
/// Nested message and enum types in `BlockSigningAuthority`.
pub mod block_signing_authority {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        V0(super::BlockSigningAuthorityV0),
    }
}
/// Present in EOSIO 2.x only
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSigningAuthorityV0 {
    #[prost(uint32, tag="1")]
    pub threshold: u32,
    #[prost(message, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<KeyWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRootMerkle {
    #[prost(uint32, tag="1")]
    pub node_count: u32,
    #[prost(bytes="vec", repeated, tag="2")]
    pub active_nodes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerToLastProduced {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub last_block_num_produced: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerToLastImpliedIrb {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub last_block_num_produced: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceipt {
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
    /// within the SignedBlock
    #[prost(uint64, tag="6")]
    pub index: u64,
    #[prost(enumeration="TransactionStatus", tag="1")]
    pub status: i32,
    #[prost(uint32, tag="2")]
    pub cpu_usage_micro_seconds: u32,
    #[prost(uint32, tag="3")]
    pub net_usage_words: u32,
    /// present if not deferred
    #[prost(message, optional, tag="5")]
    pub packed_transaction: ::core::option::Option<PackedTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackedTransaction {
    #[prost(string, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="2")]
    pub compression: u32,
    #[prost(bytes="vec", tag="3")]
    pub packed_context_free_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub packed_transaction: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub producer: ::prost::alloc::string::String,
    /// uint16
    #[prost(uint32, tag="5")]
    pub confirmed: u32,
    #[prost(string, tag="6")]
    pub previous: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub transaction_mroot: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub action_mroot: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="9")]
    pub schedule_version: u32,
    #[prost(message, repeated, tag="11")]
    pub header_extensions: ::prost::alloc::vec::Vec<Extension>,
    /// EOSIO 1.x only
    ///
    /// A change to producer schedule was reported as a `NewProducers` field on the
    /// `BlockHeader` in EOSIO 1.x. In EOSIO 2.x, when feature `WTMSIG_BLOCK_SIGNATURES`
    /// is activated, the `NewProducers` field is not present anymore and the schedule change
    /// is reported through a `BlockHeaderExtension` on the the `BlockHeader` struct.
    ///
    /// If you need to access the old value, you can
    #[prost(message, optional, tag="10")]
    pub new_producers_v1: ::core::option::Option<ProducerSchedule>,
}
/// TransactionEvent are elements that contribute to a view of the
/// whole transaction lifecycle. They can be extracted from block logs,
/// or from storage, and merged together to form an up-to-date
/// TransactionLifecycle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub block_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub block_num: u32,
    #[prost(bool, tag="4")]
    pub irreversible: bool,
    #[prost(oneof="transaction_event::Event", tags="5, 6, 7, 8, 9")]
    pub event: ::core::option::Option<transaction_event::Event>,
}
/// Nested message and enum types in `TransactionEvent`.
pub mod transaction_event {
    /// This is an implicit transaction, like `onblock` and `onerror` that is extracted
    /// separately from the block itself.
    /// TODO: does it have a receipt? It probably has
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddedInternally {
        #[prost(message, optional, tag="1")]
        pub transaction: ::core::option::Option<super::SignedTransaction>,
    }
    /// This is the transaction that is added into a block, in the list of transactions there.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Added {
        #[prost(message, optional, tag="1")]
        pub receipt: ::core::option::Option<super::TransactionReceipt>,
        #[prost(message, optional, tag="2")]
        pub transaction: ::core::option::Option<super::SignedTransaction>,
        #[prost(message, optional, tag="3")]
        pub public_keys: ::core::option::Option<super::PublicKeys>,
    }
    /// Executed contributes the traces of executions
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Executed {
        #[prost(message, optional, tag="1")]
        pub trace: ::core::option::Option<super::TransactionTrace>,
        #[prost(message, optional, tag="2")]
        pub block_header: ::core::option::Option<super::BlockHeader>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DtrxScheduled {
        #[prost(message, optional, tag="1")]
        pub created_by: ::core::option::Option<super::ExtDTrxOp>,
        #[prost(message, optional, tag="2")]
        pub transaction: ::core::option::Option<super::SignedTransaction>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DtrxCanceled {
        #[prost(message, optional, tag="1")]
        pub canceled_by: ::core::option::Option<super::ExtDTrxOp>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="5")]
        InternalAddition(AddedInternally),
        #[prost(message, tag="6")]
        Addition(Added),
        #[prost(message, tag="7")]
        Execution(Executed),
        #[prost(message, tag="8")]
        DtrxScheduling(DtrxScheduled),
        #[prost(message, tag="9")]
        DtrxCancellation(DtrxCanceled),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeys {
    #[prost(string, repeated, tag="1")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLifecycle {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="TransactionStatus", tag="2")]
    pub transaction_status: i32,
    /// FIXME: this is currently missing from our data
    #[prost(message, optional, tag="36")]
    pub transaction_receipt: ::core::option::Option<TransactionReceipt>,
    #[prost(message, optional, tag="10")]
    pub transaction: ::core::option::Option<SignedTransaction>,
    #[prost(string, repeated, tag="19")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="11")]
    pub execution_trace: ::core::option::Option<TransactionTrace>,
    #[prost(message, optional, tag="12")]
    pub execution_block_header: ::core::option::Option<BlockHeader>,
    #[prost(message, optional, tag="20")]
    pub created_by: ::core::option::Option<ExtDTrxOp>,
    #[prost(message, optional, tag="21")]
    pub canceled_by: ::core::option::Option<ExtDTrxOp>,
    // BlockState creation_block_state = 30;
    // BlockState execution_block_state = 31;
    // BlockState cancelation_block_state = 32;

    #[prost(bool, tag="33")]
    pub creation_irreversible: bool,
    #[prost(bool, tag="34")]
    pub execution_irreversible: bool,
    #[prost(bool, tag="35")]
    pub cancelation_irreversible: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransaction {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(string, repeated, tag="2")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub context_free_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<TransactionHeader>,
    #[prost(message, repeated, tag="2")]
    pub context_free_actions: ::prost::alloc::vec::Vec<Action>,
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    #[prost(message, repeated, tag="4")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHeader {
    #[prost(message, optional, tag="1")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag="2")]
    pub ref_block_num: u32,
    #[prost(uint32, tag="3")]
    pub ref_block_prefix: u32,
    #[prost(uint32, tag="4")]
    pub max_net_usage_words: u32,
    #[prost(uint32, tag="5")]
    pub max_cpu_usage_ms: u32,
    #[prost(uint32, tag="6")]
    pub delay_sec: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTrace {
    /// SHA-256 (FIPS 180-4) of the FCBUFFER-encoded packed transaction
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Reference to the block number in which this transaction was executed.
    #[prost(uint64, tag="2")]
    pub block_num: u64,
    /// Index within block's unfiltered execution traces
    #[prost(uint64, tag="26")]
    pub index: u64,
    /// Reference to the block time this transaction was executed in
    #[prost(message, optional, tag="3")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Reference to the block ID this transaction was executed in
    #[prost(string, tag="4")]
    pub producer_block_id: ::prost::alloc::string::String,
    /// Receipt of execution of this transaction
    #[prost(message, optional, tag="5")]
    pub receipt: ::core::option::Option<TransactionReceiptHeader>,
    #[prost(int64, tag="6")]
    pub elapsed: i64,
    #[prost(uint64, tag="7")]
    pub net_usage: u64,
    /// Whether this transaction was taken from a scheduled transactions pool for
    /// execution (delayed)
    #[prost(bool, tag="8")]
    pub scheduled: bool,
    /// Traces of each action within the transaction, including all notified and
    /// nested actions.
    #[prost(message, repeated, tag="9")]
    pub action_traces: ::prost::alloc::vec::Vec<ActionTrace>,
    /// Trace of a failed deferred transaction, if any.
    #[prost(message, optional, boxed, tag="10")]
    pub failed_dtrx_trace: ::core::option::Option<::prost::alloc::boxed::Box<TransactionTrace>>,
    /// Exception leading to the failed dtrx trace.
    #[prost(message, optional, tag="15")]
    pub exception: ::core::option::Option<Exception>,
    #[prost(uint64, tag="16")]
    pub error_code: u64,
    /// List of database operations this transaction entailed
    #[prost(message, repeated, tag="17")]
    pub db_ops: ::prost::alloc::vec::Vec<DbOp>,
    /// List of deferred transactions operations this transaction entailed
    #[prost(message, repeated, tag="18")]
    pub dtrx_ops: ::prost::alloc::vec::Vec<DTrxOp>,
    /// List of feature switching operations (changes to feature switches in
    /// nodeos) this transaction entailed
    #[prost(message, repeated, tag="19")]
    pub feature_ops: ::prost::alloc::vec::Vec<FeatureOp>,
    /// List of permission changes operations
    #[prost(message, repeated, tag="20")]
    pub perm_ops: ::prost::alloc::vec::Vec<PermOp>,
    /// List of RAM consumption/redemption
    #[prost(message, repeated, tag="21")]
    pub ram_ops: ::prost::alloc::vec::Vec<RamOp>,
    /// List of RAM correction operations (happens only once upon feature
    /// activation)
    #[prost(message, repeated, tag="22")]
    pub ram_correction_ops: ::prost::alloc::vec::Vec<RamCorrectionOp>,
    /// List of changes to rate limiting values
    #[prost(message, repeated, tag="23")]
    pub rlimit_ops: ::prost::alloc::vec::Vec<RlimitOp>,
    /// List of table creations/deletions
    #[prost(message, repeated, tag="24")]
    pub table_ops: ::prost::alloc::vec::Vec<TableOp>,
    /// Tree of creation, rather than execution
    #[prost(message, repeated, tag="25")]
    pub creation_tree: ::prost::alloc::vec::Vec<CreationFlatNode>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceiptHeader {
    #[prost(enumeration="TransactionStatus", tag="1")]
    pub status: i32,
    #[prost(uint32, tag="2")]
    pub cpu_usage_micro_seconds: u32,
    #[prost(uint32, tag="3")]
    pub net_usage_words: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub authorization: ::prost::alloc::vec::Vec<PermissionLevel>,
    #[prost(string, tag="4")]
    pub json_data: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionTrace {
    #[prost(string, tag="11")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, optional, tag="1")]
    pub receipt: ::core::option::Option<ActionReceipt>,
    #[prost(message, optional, tag="2")]
    pub action: ::core::option::Option<Action>,
    #[prost(bool, tag="3")]
    pub context_free: bool,
    #[prost(int64, tag="4")]
    pub elapsed: i64,
    #[prost(string, tag="5")]
    pub console: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub block_num: u64,
    #[prost(string, tag="8")]
    pub producer_block_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="10")]
    pub account_ram_deltas: ::prost::alloc::vec::Vec<AccountRamDelta>,
    /// ReturnValue has been added in EOSIO 2.1.x as something that can be returned from the execution
    /// of an action.
    ///
    /// See <https://github.com/EOSIO/eos/pull/8327>
    #[prost(bytes="vec", tag="41")]
    pub raw_return_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="42")]
    pub json_return_value: ::prost::alloc::string::String,
    #[prost(message, optional, tag="15")]
    pub exception: ::core::option::Option<Exception>,
    /// <https://github.com/EOSIO/eos/pull/7108>
    #[prost(uint64, tag="20")]
    pub error_code: u64,
    #[prost(uint32, tag="16")]
    pub action_ordinal: u32,
    #[prost(uint32, tag="17")]
    pub creator_action_ordinal: u32,
    #[prost(uint32, tag="18")]
    pub closest_unnotified_ancestor_action_ordinal: u32,
    #[prost(uint32, tag="19")]
    pub execution_index: u32,
    /// Whether this action trace was a successful match, present only when filtering was applied on block. This
    /// will be `true` if the Block `filtering_applied` is `true`, if the include CEL filter matched and
    /// if the exclude CEL filter did NOT match.
    #[prost(bool, tag="30")]
    pub filtering_matched: bool,
    /// Whether this action trace was a successful system match, present only when filtering was applied on block.
    /// This will be `true` if the Block `filtering_applied` is `true`, if the system actions include CEL filter
    /// matched, supersedes any exclude CEL filter.
    #[prost(bool, tag="31")]
    pub filtering_matched_system_action_filter: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionReceipt {
    #[prost(string, tag="1")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub digest: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub global_sequence: u64,
    #[prost(message, repeated, tag="4")]
    pub auth_sequence: ::prost::alloc::vec::Vec<AuthSequence>,
    #[prost(uint64, tag="5")]
    pub recv_sequence: u64,
    #[prost(uint64, tag="6")]
    pub code_sequence: u64,
    #[prost(uint64, tag="7")]
    pub abi_sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSequence {
    #[prost(string, tag="1")]
    pub account_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRamDelta {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub delta: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDelta {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub delta: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(uint32, tag="1")]
    pub r#type: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// FIXME: this is really just an output of the implicit transactions, isn't it? We don't have
/// other operations here.. do we?  What's the `name` anyway?
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrxOp {
    #[prost(enumeration="trx_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub transaction: ::core::option::Option<SignedTransaction>,
}
/// Nested message and enum types in `TrxOp`.
pub mod trx_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Create = 1,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Create => "OPERATION_CREATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_CREATE" => Some(Self::Create),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbOp {
    #[prost(enumeration="db_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub scope: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub primary_key: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub old_payer: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub new_payer: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub old_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub new_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="11")]
    pub old_data_json: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub new_data_json: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DBOp`.
pub mod db_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Insert = 1,
        Update = 2,
        Remove = 3,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Insert => "OPERATION_INSERT",
                Operation::Update => "OPERATION_UPDATE",
                Operation::Remove => "OPERATION_REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_INSERT" => Some(Self::Insert),
                "OPERATION_UPDATE" => Some(Self::Update),
                "OPERATION_REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RamOp {
    /// Operation is the legacy operation tag that we used initially. This is replaced
    /// by a combination of `Namespce` and `Action`.
    ///
    /// Deprecated: Use `Namespace` and `Action` instead to determine what the RAM operation represents
    #[prost(enumeration="ram_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="3")]
    pub payer: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub delta: i64,
    #[prost(uint64, tag="5")]
    pub usage: u64,
    /// Namespace representing the category the RAM operation belong to, like
    /// account, table_row, table, etc.
    ///
    /// This coupled with `action` replaces the `operation` field.
    #[prost(enumeration="ram_op::Namespace", tag="6")]
    pub namespace: i32,
    /// Namespace representing the action the RAM operation did, like
    /// add, delete or remove an object.
    ///
    /// This coupled with `action` replaces the `operation` field.
    #[prost(enumeration="ram_op::Action", tag="7")]
    pub action: i32,
    /// UniqueKey gives a unique key to the operation, this unique key is opaque,
    /// does not necessarly represents anything and should uniquely represents the
    /// RAM Operation within a given timeframe of block (a key should never overlap
    /// any other keys (per namespace), on any blocks span).
    #[prost(string, tag="8")]
    pub unique_key: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RAMOp`.
pub mod ram_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        CreateTable = 1,
        DeferredTrxAdd = 2,
        DeferredTrxCancel = 3,
        DeferredTrxPushed = 4,
        DeferredTrxRamCorrection = 5,
        DeferredTrxRemoved = 6,
        Deleteauth = 7,
        Linkauth = 8,
        Newaccount = 9,
        PrimaryIndexAdd = 10,
        PrimaryIndexRemove = 11,
        PrimaryIndexUpdate = 12,
        PrimaryIndexUpdateAddNewPayer = 13,
        PrimaryIndexUpdateRemoveOldPayer = 14,
        RemoveTable = 15,
        SecondaryIndexAdd = 16,
        SecondaryIndexRemove = 17,
        SecondaryIndexUpdateAddNewPayer = 18,
        SecondaryIndexUpdateRemoveOldPayer = 19,
        Setabi = 20,
        Setcode = 21,
        Unlinkauth = 22,
        UpdateauthCreate = 23,
        UpdateauthUpdate = 24,
        /// For newer RAM Ops that are registered by Deep Mind, their `Operation` value will be
        /// Deprecated until we remove the operation completely. Use instead the `Namespace`
        /// and `Action` fields to take a decision about what the RAM operation is doing.
        Deprecated = 25,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::CreateTable => "OPERATION_CREATE_TABLE",
                Operation::DeferredTrxAdd => "OPERATION_DEFERRED_TRX_ADD",
                Operation::DeferredTrxCancel => "OPERATION_DEFERRED_TRX_CANCEL",
                Operation::DeferredTrxPushed => "OPERATION_DEFERRED_TRX_PUSHED",
                Operation::DeferredTrxRamCorrection => "OPERATION_DEFERRED_TRX_RAM_CORRECTION",
                Operation::DeferredTrxRemoved => "OPERATION_DEFERRED_TRX_REMOVED",
                Operation::Deleteauth => "OPERATION_DELETEAUTH",
                Operation::Linkauth => "OPERATION_LINKAUTH",
                Operation::Newaccount => "OPERATION_NEWACCOUNT",
                Operation::PrimaryIndexAdd => "OPERATION_PRIMARY_INDEX_ADD",
                Operation::PrimaryIndexRemove => "OPERATION_PRIMARY_INDEX_REMOVE",
                Operation::PrimaryIndexUpdate => "OPERATION_PRIMARY_INDEX_UPDATE",
                Operation::PrimaryIndexUpdateAddNewPayer => "OPERATION_PRIMARY_INDEX_UPDATE_ADD_NEW_PAYER",
                Operation::PrimaryIndexUpdateRemoveOldPayer => "OPERATION_PRIMARY_INDEX_UPDATE_REMOVE_OLD_PAYER",
                Operation::RemoveTable => "OPERATION_REMOVE_TABLE",
                Operation::SecondaryIndexAdd => "OPERATION_SECONDARY_INDEX_ADD",
                Operation::SecondaryIndexRemove => "OPERATION_SECONDARY_INDEX_REMOVE",
                Operation::SecondaryIndexUpdateAddNewPayer => "OPERATION_SECONDARY_INDEX_UPDATE_ADD_NEW_PAYER",
                Operation::SecondaryIndexUpdateRemoveOldPayer => "OPERATION_SECONDARY_INDEX_UPDATE_REMOVE_OLD_PAYER",
                Operation::Setabi => "OPERATION_SETABI",
                Operation::Setcode => "OPERATION_SETCODE",
                Operation::Unlinkauth => "OPERATION_UNLINKAUTH",
                Operation::UpdateauthCreate => "OPERATION_UPDATEAUTH_CREATE",
                Operation::UpdateauthUpdate => "OPERATION_UPDATEAUTH_UPDATE",
                Operation::Deprecated => "OPERATION_DEPRECATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_CREATE_TABLE" => Some(Self::CreateTable),
                "OPERATION_DEFERRED_TRX_ADD" => Some(Self::DeferredTrxAdd),
                "OPERATION_DEFERRED_TRX_CANCEL" => Some(Self::DeferredTrxCancel),
                "OPERATION_DEFERRED_TRX_PUSHED" => Some(Self::DeferredTrxPushed),
                "OPERATION_DEFERRED_TRX_RAM_CORRECTION" => Some(Self::DeferredTrxRamCorrection),
                "OPERATION_DEFERRED_TRX_REMOVED" => Some(Self::DeferredTrxRemoved),
                "OPERATION_DELETEAUTH" => Some(Self::Deleteauth),
                "OPERATION_LINKAUTH" => Some(Self::Linkauth),
                "OPERATION_NEWACCOUNT" => Some(Self::Newaccount),
                "OPERATION_PRIMARY_INDEX_ADD" => Some(Self::PrimaryIndexAdd),
                "OPERATION_PRIMARY_INDEX_REMOVE" => Some(Self::PrimaryIndexRemove),
                "OPERATION_PRIMARY_INDEX_UPDATE" => Some(Self::PrimaryIndexUpdate),
                "OPERATION_PRIMARY_INDEX_UPDATE_ADD_NEW_PAYER" => Some(Self::PrimaryIndexUpdateAddNewPayer),
                "OPERATION_PRIMARY_INDEX_UPDATE_REMOVE_OLD_PAYER" => Some(Self::PrimaryIndexUpdateRemoveOldPayer),
                "OPERATION_REMOVE_TABLE" => Some(Self::RemoveTable),
                "OPERATION_SECONDARY_INDEX_ADD" => Some(Self::SecondaryIndexAdd),
                "OPERATION_SECONDARY_INDEX_REMOVE" => Some(Self::SecondaryIndexRemove),
                "OPERATION_SECONDARY_INDEX_UPDATE_ADD_NEW_PAYER" => Some(Self::SecondaryIndexUpdateAddNewPayer),
                "OPERATION_SECONDARY_INDEX_UPDATE_REMOVE_OLD_PAYER" => Some(Self::SecondaryIndexUpdateRemoveOldPayer),
                "OPERATION_SETABI" => Some(Self::Setabi),
                "OPERATION_SETCODE" => Some(Self::Setcode),
                "OPERATION_UNLINKAUTH" => Some(Self::Unlinkauth),
                "OPERATION_UPDATEAUTH_CREATE" => Some(Self::UpdateauthCreate),
                "OPERATION_UPDATEAUTH_UPDATE" => Some(Self::UpdateauthUpdate),
                "OPERATION_DEPRECATED" => Some(Self::Deprecated),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Namespace {
        Unknown = 0,
        Abi = 1,
        Account = 2,
        Auth = 3,
        AuthLink = 4,
        Code = 5,
        DeferredTrx = 6,
        SecondaryIndex = 7,
        Table = 8,
        TableRow = 9,
    }
    impl Namespace {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Namespace::Unknown => "NAMESPACE_UNKNOWN",
                Namespace::Abi => "NAMESPACE_ABI",
                Namespace::Account => "NAMESPACE_ACCOUNT",
                Namespace::Auth => "NAMESPACE_AUTH",
                Namespace::AuthLink => "NAMESPACE_AUTH_LINK",
                Namespace::Code => "NAMESPACE_CODE",
                Namespace::DeferredTrx => "NAMESPACE_DEFERRED_TRX",
                Namespace::SecondaryIndex => "NAMESPACE_SECONDARY_INDEX",
                Namespace::Table => "NAMESPACE_TABLE",
                Namespace::TableRow => "NAMESPACE_TABLE_ROW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NAMESPACE_UNKNOWN" => Some(Self::Unknown),
                "NAMESPACE_ABI" => Some(Self::Abi),
                "NAMESPACE_ACCOUNT" => Some(Self::Account),
                "NAMESPACE_AUTH" => Some(Self::Auth),
                "NAMESPACE_AUTH_LINK" => Some(Self::AuthLink),
                "NAMESPACE_CODE" => Some(Self::Code),
                "NAMESPACE_DEFERRED_TRX" => Some(Self::DeferredTrx),
                "NAMESPACE_SECONDARY_INDEX" => Some(Self::SecondaryIndex),
                "NAMESPACE_TABLE" => Some(Self::Table),
                "NAMESPACE_TABLE_ROW" => Some(Self::TableRow),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        Unknown = 0,
        Add = 1,
        Cancel = 2,
        Correction = 3,
        Push = 4,
        Remove = 5,
        Update = 6,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unknown => "ACTION_UNKNOWN",
                Action::Add => "ACTION_ADD",
                Action::Cancel => "ACTION_CANCEL",
                Action::Correction => "ACTION_CORRECTION",
                Action::Push => "ACTION_PUSH",
                Action::Remove => "ACTION_REMOVE",
                Action::Update => "ACTION_UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNKNOWN" => Some(Self::Unknown),
                "ACTION_ADD" => Some(Self::Add),
                "ACTION_CANCEL" => Some(Self::Cancel),
                "ACTION_CORRECTION" => Some(Self::Correction),
                "ACTION_PUSH" => Some(Self::Push),
                "ACTION_REMOVE" => Some(Self::Remove),
                "ACTION_UPDATE" => Some(Self::Update),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RamCorrectionOp {
    #[prost(string, tag="1")]
    pub correction_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub unique_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub payer: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub delta: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOp {
    #[prost(enumeration="table_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="3")]
    pub payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub scope: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub table_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TableOp`.
pub mod table_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Insert = 1,
        Remove = 2,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Insert => "OPERATION_INSERT",
                Operation::Remove => "OPERATION_REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_INSERT" => Some(Self::Insert),
                "OPERATION_REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DTrxOp {
    #[prost(enumeration="d_trx_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub sender_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub published_at: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub delay_until: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub expiration_at: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub transaction: ::core::option::Option<SignedTransaction>,
}
/// Nested message and enum types in `DTrxOp`.
pub mod d_trx_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Create = 1,
        PushCreate = 2,
        Failed = 3,
        Cancel = 4,
        ModifyCancel = 5,
        ModifyCreate = 6,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Create => "OPERATION_CREATE",
                Operation::PushCreate => "OPERATION_PUSH_CREATE",
                Operation::Failed => "OPERATION_FAILED",
                Operation::Cancel => "OPERATION_CANCEL",
                Operation::ModifyCancel => "OPERATION_MODIFY_CANCEL",
                Operation::ModifyCreate => "OPERATION_MODIFY_CREATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_CREATE" => Some(Self::Create),
                "OPERATION_PUSH_CREATE" => Some(Self::PushCreate),
                "OPERATION_FAILED" => Some(Self::Failed),
                "OPERATION_CANCEL" => Some(Self::Cancel),
                "OPERATION_MODIFY_CANCEL" => Some(Self::ModifyCancel),
                "OPERATION_MODIFY_CREATE" => Some(Self::ModifyCreate),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtDTrxOp {
    #[prost(string, tag="1")]
    pub source_transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_num: u64,
    #[prost(string, tag="3")]
    pub block_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub dtrx_op: ::core::option::Option<DTrxOp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureOp {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(string, tag="3")]
    pub feature_digest: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub feature: ::core::option::Option<Feature>,
}
/// Nested message and enum types in `FeatureOp`.
pub mod feature_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Unknown = 0,
        PreActivate = 1,
        Activate = 2,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unknown => "KIND_UNKNOWN",
                Kind::PreActivate => "KIND_PRE_ACTIVATE",
                Kind::Activate => "KIND_ACTIVATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_UNKNOWN" => Some(Self::Unknown),
                "KIND_PRE_ACTIVATE" => Some(Self::PreActivate),
                "KIND_ACTIVATE" => Some(Self::Activate),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreationFlatNode {
    #[prost(int32, tag="1")]
    pub creator_action_index: i32,
    #[prost(uint32, tag="2")]
    pub execution_action_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermOp {
    #[prost(enumeration="perm_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    #[prost(message, optional, tag="8")]
    pub old_perm: ::core::option::Option<PermissionObject>,
    #[prost(message, optional, tag="9")]
    pub new_perm: ::core::option::Option<PermissionObject>,
}
/// Nested message and enum types in `PermOp`.
pub mod perm_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Insert = 1,
        Update = 2,
        Remove = 3,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Insert => "OPERATION_INSERT",
                Operation::Update => "OPERATION_UPDATE",
                Operation::Remove => "OPERATION_REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_INSERT" => Some(Self::Insert),
                "OPERATION_UPDATE" => Some(Self::Update),
                "OPERATION_REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionObject {
    /// Id represents the EOSIO internal id of this permission object.
    #[prost(uint64, tag="10")]
    pub id: u64,
    /// ParentId represents the EOSIO internal id of the parent's of this permission object.
    #[prost(uint64, tag="11")]
    pub parent_id: u64,
    /// Owner is the account for which this permission was set
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Name is the permission's name this permission object is known as.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub last_updated: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub authority: ::core::option::Option<Authority>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub required_auth: ::core::option::Option<Authority>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(uint32, tag="1")]
    pub threshold: u32,
    #[prost(message, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<KeyWeight>,
    #[prost(message, repeated, tag="3")]
    pub accounts: ::prost::alloc::vec::Vec<PermissionLevelWeight>,
    #[prost(message, repeated, tag="4")]
    pub waits: ::prost::alloc::vec::Vec<WaitWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyWeight {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub weight: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionLevel {
    #[prost(string, tag="1")]
    pub actor: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub permission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionLevelWeight {
    #[prost(message, optional, tag="1")]
    pub permission: ::core::option::Option<PermissionLevel>,
    #[prost(uint32, tag="2")]
    pub weight: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitWeight {
    #[prost(uint32, tag="1")]
    pub wait_sec: u32,
    #[prost(uint32, tag="2")]
    pub weight: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlimitOp {
    #[prost(enumeration="rlimit_op::Operation", tag="1")]
    pub operation: i32,
    #[prost(oneof="rlimit_op::Kind", tags="2, 3, 4, 5")]
    pub kind: ::core::option::Option<rlimit_op::Kind>,
}
/// Nested message and enum types in `RlimitOp`.
pub mod rlimit_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unknown = 0,
        Insert = 1,
        Update = 2,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unknown => "OPERATION_UNKNOWN",
                Operation::Insert => "OPERATION_INSERT",
                Operation::Update => "OPERATION_UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNKNOWN" => Some(Self::Unknown),
                "OPERATION_INSERT" => Some(Self::Insert),
                "OPERATION_UPDATE" => Some(Self::Update),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="2")]
        State(super::RlimitState),
        #[prost(message, tag="3")]
        Config(super::RlimitConfig),
        #[prost(message, tag="4")]
        AccountLimits(super::RlimitAccountLimits),
        #[prost(message, tag="5")]
        AccountUsage(super::RlimitAccountUsage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlimitState {
    #[prost(message, optional, tag="1")]
    pub average_block_net_usage: ::core::option::Option<UsageAccumulator>,
    #[prost(message, optional, tag="2")]
    pub average_block_cpu_usage: ::core::option::Option<UsageAccumulator>,
    #[prost(uint64, tag="3")]
    pub pending_net_usage: u64,
    #[prost(uint64, tag="4")]
    pub pending_cpu_usage: u64,
    #[prost(uint64, tag="5")]
    pub total_net_weight: u64,
    #[prost(uint64, tag="6")]
    pub total_cpu_weight: u64,
    #[prost(uint64, tag="7")]
    pub total_ram_bytes: u64,
    #[prost(uint64, tag="8")]
    pub virtual_net_limit: u64,
    #[prost(uint64, tag="9")]
    pub virtual_cpu_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlimitConfig {
    #[prost(message, optional, tag="1")]
    pub cpu_limit_parameters: ::core::option::Option<ElasticLimitParameters>,
    #[prost(message, optional, tag="2")]
    pub net_limit_parameters: ::core::option::Option<ElasticLimitParameters>,
    #[prost(uint32, tag="3")]
    pub account_cpu_usage_average_window: u32,
    #[prost(uint32, tag="4")]
    pub account_net_usage_average_window: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlimitAccountLimits {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub pending: bool,
    #[prost(int64, tag="3")]
    pub net_weight: i64,
    #[prost(int64, tag="4")]
    pub cpu_weight: i64,
    #[prost(int64, tag="5")]
    pub ram_bytes: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlimitAccountUsage {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub net_usage: ::core::option::Option<UsageAccumulator>,
    #[prost(message, optional, tag="3")]
    pub cpu_usage: ::core::option::Option<UsageAccumulator>,
    #[prost(uint64, tag="4")]
    pub ram_usage: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageAccumulator {
    #[prost(uint32, tag="1")]
    pub last_ordinal: u32,
    #[prost(uint64, tag="2")]
    pub value_ex: u64,
    #[prost(uint64, tag="3")]
    pub consumed: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElasticLimitParameters {
    #[prost(uint64, tag="1")]
    pub target: u64,
    #[prost(uint64, tag="2")]
    pub max: u64,
    #[prost(uint32, tag="3")]
    pub periods: u32,
    #[prost(uint32, tag="4")]
    pub max_multiplier: u32,
    #[prost(message, optional, tag="5")]
    pub contract_rate: ::core::option::Option<Ratio>,
    #[prost(message, optional, tag="6")]
    pub expand_rate: ::core::option::Option<Ratio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ratio {
    #[prost(uint64, tag="1")]
    pub numerator: u64,
    #[prost(uint64, tag="2")]
    pub denominator: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exception {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub stack: ::prost::alloc::vec::Vec<exception::LogMessage>,
}
/// Nested message and enum types in `Exception`.
pub mod exception {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogMessage {
        #[prost(message, optional, tag="1")]
        pub context: ::core::option::Option<LogContext>,
        #[prost(string, tag="2")]
        pub format: ::prost::alloc::string::String,
        /// This is actually a Pair<string, any> in C++ which get serialized usually
        /// as a JSON object. However, it seems some string sequences could be
        /// invalid UTF-8 characters. As such, we decided to use a bytes array. Can
        /// be interpreted as a UTF-8 string containing JSON, just be ready to
        /// handle UTF-8 errors if they arise.
        #[prost(bytes="vec", tag="4")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogContext {
        #[prost(string, tag="1")]
        pub level: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub file: ::prost::alloc::string::String,
        #[prost(int32, tag="3")]
        pub line: i32,
        #[prost(string, tag="4")]
        pub method: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub hostname: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub thread_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="7")]
        pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, boxed, tag="8")]
        pub context: ::core::option::Option<::prost::alloc::boxed::Box<LogContext>>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag="1")]
    pub feature_digest: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub subjective_restrictions: ::core::option::Option<SubjectiveRestrictions>,
    #[prost(string, tag="3")]
    pub description_digest: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub dependencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub protocol_feature_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub specification: ::prost::alloc::vec::Vec<Specification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectiveRestrictions {
    #[prost(bool, tag="1")]
    pub enabled: bool,
    #[prost(bool, tag="2")]
    pub preactivation_required: bool,
    #[prost(string, tag="3")]
    pub earliest_allowed_activation_time: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Specification {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// AccountCreation represents the time and place where an account was created on an EOSIO chain. The block referenced is expected to be irreversible.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCreationRef {
    /// Account being created
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Account that created the `account`
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
    /// At which block number this happened
    #[prost(uint64, tag="3")]
    pub block_num: u64,
    /// At which block ID this happened
    #[prost(string, tag="4")]
    pub block_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="6")]
    pub transaction_id: ::prost::alloc::string::String,
}
/// HeaderOnlyBlock is used to optimally unpack the \[Block\] structure (note the
/// corresponding message number for the `header` field) while consuming less
/// memory, when only the `header` is desired.
///
/// WARN: this is a client-side optimization pattern and should be moved in the
/// consuming code.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderOnlyBlock {
    /// id is the block's hash.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Number is the block's height at which this block was mined.
    #[prost(uint32, tag="2")]
    pub number: u32,
    /// Header contain's the block's header information like its parent hash, the merkel root hash
    /// and all other information the form a block.
    #[prost(message, optional, tag="4")]
    pub header: ::core::option::Option<BlockHeader>,
    /// A blockroot Merkle tree is a data structure that uses hash functions to organize and 
    /// verify the integrity of data blocks in a blockchain.
    /// Each leaf node contains a hash of a data block, while each non-leaf node contains the hash of its child nodes.
    /// The root node contains a single hash that represents all the data in the tree,
    /// allowing for efficient verification of data integrity by only checking the root hash.
    #[prost(message, optional, tag="11")]
    pub blockroot_merkle: ::core::option::Option<BlockRootMerkle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTraceWithBlockRef {
    #[prost(message, optional, tag="1")]
    pub trace: ::core::option::Option<TransactionTrace>,
    #[prost(message, optional, tag="2")]
    pub block_ref: ::core::option::Option<BlockRef>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRef {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub number: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockReversibility {
    /// there is no block
    BlockreversibilityNone = 0,
    BlockreversibilityReversible = 1,
    BlockreversibilityIrreversible = 2,
    BlockreversibilityStale = 3,
    /// behind lib, but we have not confirmed irreversibility
    BlockreversibilityMaybestale = 4,
}
impl BlockReversibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockReversibility::BlockreversibilityNone => "BLOCKREVERSIBILITY_NONE",
            BlockReversibility::BlockreversibilityReversible => "BLOCKREVERSIBILITY_REVERSIBLE",
            BlockReversibility::BlockreversibilityIrreversible => "BLOCKREVERSIBILITY_IRREVERSIBLE",
            BlockReversibility::BlockreversibilityStale => "BLOCKREVERSIBILITY_STALE",
            BlockReversibility::BlockreversibilityMaybestale => "BLOCKREVERSIBILITY_MAYBESTALE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCKREVERSIBILITY_NONE" => Some(Self::BlockreversibilityNone),
            "BLOCKREVERSIBILITY_REVERSIBLE" => Some(Self::BlockreversibilityReversible),
            "BLOCKREVERSIBILITY_IRREVERSIBLE" => Some(Self::BlockreversibilityIrreversible),
            "BLOCKREVERSIBILITY_STALE" => Some(Self::BlockreversibilityStale),
            "BLOCKREVERSIBILITY_MAYBESTALE" => Some(Self::BlockreversibilityMaybestale),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    TransactionstatusNone = 0,
    TransactionstatusExecuted = 1,
    TransactionstatusSoftfail = 2,
    TransactionstatusHardfail = 3,
    TransactionstatusDelayed = 4,
    TransactionstatusExpired = 5,
    TransactionstatusUnknown = 6,
    TransactionstatusCanceled = 7,
}
impl TransactionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionStatus::TransactionstatusNone => "TRANSACTIONSTATUS_NONE",
            TransactionStatus::TransactionstatusExecuted => "TRANSACTIONSTATUS_EXECUTED",
            TransactionStatus::TransactionstatusSoftfail => "TRANSACTIONSTATUS_SOFTFAIL",
            TransactionStatus::TransactionstatusHardfail => "TRANSACTIONSTATUS_HARDFAIL",
            TransactionStatus::TransactionstatusDelayed => "TRANSACTIONSTATUS_DELAYED",
            TransactionStatus::TransactionstatusExpired => "TRANSACTIONSTATUS_EXPIRED",
            TransactionStatus::TransactionstatusUnknown => "TRANSACTIONSTATUS_UNKNOWN",
            TransactionStatus::TransactionstatusCanceled => "TRANSACTIONSTATUS_CANCELED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTIONSTATUS_NONE" => Some(Self::TransactionstatusNone),
            "TRANSACTIONSTATUS_EXECUTED" => Some(Self::TransactionstatusExecuted),
            "TRANSACTIONSTATUS_SOFTFAIL" => Some(Self::TransactionstatusSoftfail),
            "TRANSACTIONSTATUS_HARDFAIL" => Some(Self::TransactionstatusHardfail),
            "TRANSACTIONSTATUS_DELAYED" => Some(Self::TransactionstatusDelayed),
            "TRANSACTIONSTATUS_EXPIRED" => Some(Self::TransactionstatusExpired),
            "TRANSACTIONSTATUS_UNKNOWN" => Some(Self::TransactionstatusUnknown),
            "TRANSACTIONSTATUS_CANCELED" => Some(Self::TransactionstatusCanceled),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
