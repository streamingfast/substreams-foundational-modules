# Injective Foundational Modules

The Injective Foundational modules are Substreams modules extracting common data from the Injective blockchain.

## Modules

### map_transactions

The `map_transactions` module extracts all the transactions from the Injective blockchain, providing useful information, such as _messages_, _signatures_ or _events_.

In the Injective data model, _messages_ are Protobufs that can be decoded if you have the corresponding Protobuf model. In this Substreams, some of the most common Protobufs are provided in the `substreams.yaml`:

```yaml
protobuf:
  descriptorSets:
    - module: buf.build/cosmos/cosmos-sdk
      version: v0.50.0
      symbols:
        - cosmos.tx.v1beta1.Tx
        - cosmos.authz.v1beta1.MsgExec
        - cosmos.bank.v1beta1.MsgSend
        - cosmos.bank.v1beta1.MsgMultiSend
        - cosmos.crisis.v1beta1.MsgVerifyInvariant
        - cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward
        - cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission
        - cosmos.distribution.v1beta1.MsgSetWithdrawAddress
        - cosmos.distribution.v1beta1.MsgFundCommunityPool
        - cosmos.evidence.v1beta1.MsgSubmitEvidence
        - cosmos.gov.v1beta1.MsgSubmitProposal
        - cosmos.gov.v1beta1.MsgVote
        - cosmos.gov.v1beta1.MsgDeposit
        - cosmos.slashing.v1beta1.MsgUnjail
```

Using the `substreams protogen` command you can generate the corresponding Rust code, and decode the _message_ data of the transaction:

```rust
fn extract_messages(messages: Vec<Any>) -> Vec<Message> {
    return messages
        .iter()
        .map(|message| {
            let message_as_u8 = &message.value[..];

            if message.type_url == "cosmos.authz.v1beta1.MsgExec" {
                if let Ok(msg_exec) = <MsgExec as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgExec(msg_exec));
                }
            }
            if message.type_url == "cosmos.bank.v1beta1.MsgSend" {
                if let Ok(msg_send) = <MsgSend as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgSend(msg_send));
                }
            }
            if message.type_url == "cosmos.bank.v1beta1.MsgMultiSend" {
                if let Ok(msg_multi_send) = <MsgMultiSend as prost::Message>::decode(message_as_u8) {
                    return build_message(Value::MsgMultiSend(msg_multi_send));
                }
            }
            // ...output omitted...
        });
}
```

## Getting Started

### Generate Protobufs

```bash
make protogen
```

### Build Substreams

```bash
make build
```

### Run Substreams

```bash
substreams run substreams.yaml map_transactions -e mainnet.injective.streamingfast.io:443 --start-block=69746551 --stop-block=+1
```

### Package as .spkg

```bash
make package
```
