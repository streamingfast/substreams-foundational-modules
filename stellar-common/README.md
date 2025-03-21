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
