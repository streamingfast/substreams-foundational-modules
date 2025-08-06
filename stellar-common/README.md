# Stellar Foundational Modules

The **Stellar Foundational Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Stellar blockchain, such as transactions or operations.

## Modules

### map_transactions

This module retrieves all the **NOT** failed transactions, without any more filtering.

### index_transactions

This module creates a cache of transactions based on:
- The _source account_ of the transaction.
- The _source accounts_ of every operation contained within the transaction.

You can use this module as a `blockFilter` to filter transactions based on the parameters specified above:

```yaml
  - name: my_module
    ...
    blockFilter:
      module: index_transactions
      query:
        string: (source_account:source_account1 || source_account:source_account2)
```

### filtered_transactions

This module uses the `index_transactions` cache to match the filtered transactions based on the parameters passed as input to the module.

You can directly _use_ this module to retrieve filtered transactions:

```yaml
modules:
    - name: my_module
      use: stellar_common:filtered_transactions

params:
    my_module: source_account:account1
```

### map_operations

This module decodes **some** of the Stellar operations. These are the operation supported:

```rust
&Op::CreateAccount(_) => "create_account",
&Op::AccountMerge(_) => "account_merge",
&Op::Payment(_) => "payment",
&Op::CreateClaimableBalance(_) => "create_claimable_balance",
&Op::ClaimClaimableBalance(_) => "claim_claimable_balance",
&Op::Clawback(_) => "clawback",
&Op::ClawbackClaimableBalance(_) => "clawback_claimable_balance",
&Op::AllowTrust(_) => "allow_trust",
&Op::SetTrustLineFlags(_) => "set_trust_line_flags",
&Op::LiquidityPoolDeposit(_) => "liquidity_pool_deposit",
&Op::LiquidityPoolWithdraw(_) => "liquidity_pool_withdraw",
&Op::ManageBuyOffer(_) => "manage_buy_offer",
&Op::ManageSellOffer(_) => "manage_sell_offer",
&Op::CreatePassiveSellOffer(_) => "create_passive_sell_offer",
&Op::PathPaymentStrictSend(_) => "path_payment_strict_send",
&Op::PathPaymentStrictReceive(_) => "path_payment_strict_receive",
```

### index_operations

This module creates a cache of operations, which you can use to filter by the operation name.

You can use this module as a `blockFilter` to filter operations:

```yaml
  - name: my_module
    ...
    blockFilter:
      module: index_operations
      query:
        string: (operation:payment || operation:create_account)
```

### filtered_operations

This module uses the `index_operations` cache to do the actual filtering of the operations.

You can directly _use_ this module to retrieve filtered operations:

```yaml
modules:
    - name: my_module
      use: stellar_common:filtered_operations

params:
    my_module: (operation:payment || operation:create_account)
```

### map_events

This module extracts Stellar Soroban contract events from transaction metadata. It decodes events from Soroban metadata and maps them to a structured format including:
- Contract ID (if available)
- Event type (Diagnostic, System, or Contract)
- Topics (JSON-encoded topic data)
- Data (JSON-encoded event data)

### index_events

This module creates a cache of events that allows filtering based on event properties and topic content.

The indexing extracts keys from:
- Event type: `type:Contract`, `type:System`, `type:Diagnostic`
- Contract ID: `contract_id:abc123...`
- Topic data: `topic:symbol:transfer`, `topic:address:CB7FKGSTHP...`

You can use this module as a `blockFilter` to filter events:

```yaml
  - name: my_module
    ...
    blockFilter:
      module: index_events
      query:
        string: (type:Contract && contract_id:abc123)
```

### filtered_events

This module uses the `index_events` cache to filter events based on the specified query parameters.

#### Topic Mapping to Keys

Topics are parsed as JSON objects and their key-value pairs are mapped to searchable keys with the format `topic:{key}:{value}`. Only string values are indexed:

- `{"symbol":"transfer"}` → `topic:symbol:transfer`
- `{"address":"CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV"}` → `topic:address:CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV`
- `{"string":"USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"}` → `topic:string:USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN`

Non-string values (numbers, booleans, arrays) are ignored during indexing. Invalid JSON topics are also skipped.

You can directly _use_ this module to retrieve filtered events:

```yaml
modules:
    - name: my_module
      use: stellar_common:filtered_events

params:
    my_module: type:Contract && topic:symbol:transfer
```

Example queries:
- Filter by contract and symbol: `contract_id:abc123 && topic:symbol:transfer`
- Filter by multiple addresses: `topic:address:CB7FKGSTHP75ORTIZGGMVUTQLEMVTSEOI4QORQPCABJSGTAATDFCE2YV || topic:address:CB3JAPDEIMA3OOSALUHLYRGM2QTXGVD3EASALPFMVEU2POLLULJBT2XN`
- Filter by event type: `type:Contract`
