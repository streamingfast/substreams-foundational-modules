# Token Metadata Foundational Store

A Substreams module that extracts token metadata (name, symbol, decimals) from EVM blockchains and outputs it in a format compatible with the [Substreams Foundational Store](https://github.com/streamingfast/substreams-foundational-store/tree/develop).

## Overview

This project was created for use with the **Substreams Foundational Store** from StreamingFast, which provides a high-performance, multi-backend key-value storage system designed for Substreams data ingestion and serving.

The substream imports the [ERC20 Metadata substream created by Pinax](https://github.com/pinax-network/substreams-evm-tokens) for the [TokenAPI](https://token-api.thegraph.com/), extending it to output data in the Foundational Store format.

> 📖 **[View the full documentation on substreams.dev](https://docs.substreams.dev/how-to-guides/foundational-stores/erc20-token-metadata)**
