## Antelope Foundational Substreams Modules

Foundational modules let developers build substreams modules on top, to minimize bytes billed and improve performance by filtering out non-relevant parts of blocks and by skipping irrelevant chunks of blocks from the stream.

There are three types of foundational modules:
- `all-*` - modules with outputs that include certain messages in a block only, such as `Actions`,  `Transactions`, minimizing bytes billed
- `filtered-*` - modules with outputs defined by a query. When indexed, will skip large chunks of empty blocks improving performance
- `index-*` - index modules that index blocks with related keys

Antelope foundational modules include:
- `all_transactions` - all transactions in a block
- `all_actions` - all flattened actions in a block
- `index_actions` - index blocks with relevant action-related keys
- `filtered_actions` - flattened actions in a block filtered based on the query
- `filtered_transactions` - transactions in a block filtered based on the query

Filtered modules with queries take advantage of backend indexing, so every subsequent request with the same query will make the backend stream the response much faster skipping all the empty blocks.
Common use cases:
- Stream all USDT token transfers
- Stream all Defibox pool creation events
- Stream all AtomicHub NFT drops
- etc

### How it works
Let's say you want to receive all AtomicAssets NFT create collection events starting from block 370,000,000.
Send a substreams request with the desired query as parameter. You can use a gRPC client, substreams sink, or substreams CLI:
```bash
> substreams gui -e eos.substreams.pinax.network:443 https://spkg.io/pinax-network/antelope-common-v0.3.0.spkg filtered_actions -s 370000000 -p filtered_actions="code:atomicassets && action:createcol" --production-mode
```
If the request with this query hasn't been run before, substreams backend will start the indexing process and you should start seeing new events. If the request has been run before, you should start seeing sale actions right away jumping over any empty chunks of blocks when there were no sales.
Note, we used `--production-mode` flag - this ensures backend writes indexes on disk so they can be re-used in the future. 

### Example queries
- `code:mycontract`
- `code:tethertether && action:issue`
- `code:eosio.token && action:transfer && (data.to:myaccount || data.from:myaccount)`
- `auth:myaccount@active`
- `code:atomicassets && action:logmint`

### Available query fields
These are the expressions that can be used in queries:
- `action:<action_name>` - action name
- `code:<account>` - smart contract account name
- `receiver:<account>` - action receiver account
- `auth:` - account which authority was used to sign the action, i.e.
  - `auth:<account>` - account with any permission
  - `auth:<account>@<permission>` - account with a specific permission
- `input:true` - will match only the top-level actions
- `data.<field>:` - will decode and match action parameters (doesn't support nested objects). Some examples:
  - `data.from:myaccount`
  - `data.memo:"your daily staking rewards"`

Queries can include `&&` and `||` logical operands, as well as `(` and `)` parenthesis.


### Release
v0.3.0: https://substreams.dev/pinax-network/antelope-common/v0.3.0


### Usage
```bash
substreams gui -e eos.substreams.pinax.network:443 https://spkg.io/pinax-network/antelope-common-v0.3.0.spkg filtered_actions -s -10000 -p filtered_actions="code:tethertether && data.to:swap.defi" --production-mode
```

### Known issues
- Your query needs to be normalized, i.e `code:eosio.token` and `(code:eosio.token)` are different queries and will trigger re-indexing