# Vara Mainnet Foundational Modules

In this repository, you will find all the foundational modules containing various mappers and indexed Substreams. These foundational modules allow users to easily consume any `Extrinsic` or `Even ` from the Vara Mainnet blockchain. The Substreams were written in Golang using `tinygo`, for [reference](https://tinygo.org/) and `wizer`, for [reference](https://github.com/bytecodealliance/wizer?tab=readme-ov-file#install)

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

### `filtered_extrinsics`

This module is probably the most interesting one to use. The `filtered_extrinsics` module will output only the extrinsics of the query passed in `--params`.

```bash
substreams gui
```

### Contributing

For more information on how to contribute refer to [CONTRIBUTING.md](CONTRIBUTING.md)
