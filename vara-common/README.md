# Vara Mainnet Foundational Modules

In this repository, you will find all the foundational modules containing various mappers and indexed Substreams. These foundational modules allow users to easily consume any `Extrinsic` or `Even` from the Vara Mainnet blockchain. The Substreams were written in Golang using `tinygo`, for [reference](https://tinygo.org/) and `wizer`, for [reference](https://github.com/bytecodealliance/wizer?tab=readme-ov-file#install)

## Usage

```bash
substreams gui
```

The raw `sf.gear.type.v1.Block` will contain the `Extrinsics` and the `Events` of each block in raw format. Run `map_decoded_block` to get the augmented block with all `Extrinsics` and `Events` decoded.

## Modules

### `all_extrinsics`

Output the extrinsics decoded with this format `sf.substreams.gear.type.v1.Extrinsics`.

```bash
substreams gui
```

### `index_extrinsics`

The keys for the extrinsics will follow this format: `extrinsic:${CallName}` and `extrinsic:${CallName}:event:${eventName}`.

You can visit this [page](https://vara.subscan.io/runtime) to see all the Runtime Modules. The extrinsics are named Call Functions on the Runtime Modules. For example, you can see the extrinsics for `set` `Timestamp` module [here](https://vara.subscan.io/runtime/Timestamp).

So for example, for the `Timestamp` - `set`, you will have keys of this format: `extrinsic:Timestamp.set`.

### `filtered_extrinsics`

This module is probably the most interesting one to use. The `filtered_extrinsics` module will output only the extrinsics of the query passed in `--params`.

```bash
substreams gui
```

### Contributing

For more information on how to contribute refer to [CONTRIBUTING.md](CONTRIBUTING.md)
