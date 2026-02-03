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

## Memory Allocation Profiling

To see memory allocations during benchmarks, you have three options:

### Option 1: DHAT Heap Profiler (Most Detailed)

Run the dedicated memory benchmark:

```bash
cargo bench --bench memory_benchmarks
```

This will:
- Show allocation statistics for each operation
- Generate a `dhat-heap.json` file with detailed profiling data
- View the results at: https://nnethercote.github.io/dh_view/dh_view.html

The DHAT profiler shows:
- Total allocations and deallocations
- Memory leaks
- Peak memory usage
- Allocation hotspots
- Lifetimes of allocations

### Option 2: Criterion with Allocation Counter

Run benchmarks with built-in allocation counting:

```bash
cargo bench --bench criterion_with_alloc_counter
```

This will display allocation statistics in stderr:
- Bytes allocated per iteration
- Number of allocations per iteration
- Bytes deallocated per iteration
- Number of deallocations per iteration

Example output:
```
block_clone stats (avg per iteration):
  Bytes allocated: 524288
  Bytes deallocated: 524288
  Allocations: 128
  Deallocations: 128
```

### Option 3: Valgrind/Massif (Linux only)

For system-level memory profiling:

```bash
# Install valgrind (Linux)
sudo apt-get install valgrind

# Run with massif
valgrind --tool=massif --massif-out-file=massif.out \
  cargo bench --bench solana_common_benchmarks -- --profile-time=5

# Visualize results
ms_print massif.out
```

### Option 4: Heaptrack (Linux only)

For graphical memory profiling:

```bash
# Install heaptrack
sudo apt-get install heaptrack

# Run benchmarks under heaptrack
heaptrack cargo bench --bench solana_common_benchmarks

# Open GUI to analyze
heaptrack_gui heaptrack.*.gz
```

### Option 5: Instruments (macOS only)

For macOS users:

```bash
# Build benchmarks
cargo bench --bench solana_common_benchmarks --no-run

# Find the binary path
find target/release -name "solana_common_benchmarks*" -type f

# Run with Instruments
instruments -t "Allocations" path/to/benchmark/binary -- --bench
```

## Interpreting Memory Results

When analyzing memory allocations:

1. **Total bytes allocated** - How much memory the operation needs
2. **Allocation count** - Number of separate allocations (fewer is often better)
3. **Peak memory** - Maximum memory used at any point
4. **Memory retained** - Memory that isn't deallocated (potential leaks)

### Common Optimization Strategies

Based on allocation profiling:

- **Reduce cloning**: Use references or `Cow` types where possible
- **Pre-allocate collections**: Use `Vec::with_capacity()` if size is known
- **Avoid string allocations**: Use `&str` instead of `String` when possible
- **Batch allocations**: Allocate once and reuse rather than repeated small allocations

## Dependencies

The benchmarks require:
- `criterion = "0.5"` with HTML reports enabled
- `dhat = "0.3"` for memory profiling
- Test data files must be present in `src/testdata/`
