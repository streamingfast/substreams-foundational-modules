# Solana Common Benchmarks

This directory contains performance benchmarks for the `solana-common` module using the Criterion benchmarking framework.

## Running Benchmarks

To run all benchmarks:

```bash
cargo bench
```

To run a specific benchmark:

```bash
cargo bench --bench solana_common_benchmarks -- <benchmark_name>
```

For example:

```bash
cargo bench --bench solana_common_benchmarks -- blocks_without_votes
```

## Available Benchmarks

### Block Processing

1. **blocks_without_votes** - Measures performance of filtering out vote transactions from blocks
2. **block_clone** - Measures the cost of cloning a full block
3. **transaction_count** - Baseline measurement for accessing transaction count

### Transaction Filtering

4. **transactions_by_programid_without_votes** - Filters transactions by program ID using expression matching
5. **transactions_by_programid_and_account** - Filters transactions by both program ID and account address
6. **transaction_retain_with_filter** - Measures the retain operation with program ID filtering

### Key Extraction

7. **transaction_program_and_account_keys** - Extracts all program and account keys from a single transaction
8. **program_ids_without_votes** - Extracts all program IDs from a block
9. **program_ids_and_accounts_without_votes** - Extracts both program IDs and account keys from all transactions in a block

### Instruction Walking

10. **walk_instructions_single_transaction** - Counts instructions in a single transaction
11. **walk_all_instructions_in_block** - Counts all instructions across all transactions in a block

## Benchmark Results

Criterion will generate detailed HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` in a browser to view:

- Performance statistics (mean, median, standard deviation)
- Plots showing performance distributions
- Comparisons with previous runs (if available)

## Test Data

The benchmarks use real Solana mainnet block data located at:
- `./src/testdata/solana_mainnet_313000000.binpb.base64`

This ensures benchmarks reflect real-world performance characteristics.

## Interpreting Results

When analyzing benchmark results, consider:

- **Throughput**: Operations per second (higher is better)
- **Latency**: Time per operation (lower is better)
- **Variance**: Lower standard deviation indicates more consistent performance
- **Comparisons**: Criterion automatically compares with previous runs to detect regressions

## Continuous Integration

To run benchmarks in CI without the full statistical analysis:

```bash
cargo bench --bench solana_common_benchmarks -- --quick
```

## Performance Tips

Based on the benchmarks, the most expensive operations are:

1. Block cloning - Consider using references when possible
2. Walking all instructions - Cache results if used multiple times
3. Expression matching with complex queries - Simplify queries when possible

## Dependencies

The benchmarks require:
- `criterion = "0.5"` with HTML reports enabled
- Test data files must be present in `src/testdata/`
