# Benchmark Summary - Solana Common

## Available Benchmark Suites

This module provides three complementary benchmark suites:

### 1. Performance Benchmarks (solana_common_benchmarks)
**Focus:** Execution time and throughput
**File:** `benches/solana_common_benchmarks.rs`

#### Run Command:
\`\`\`bash
cargo bench --bench solana_common_benchmarks
\`\`\`

#### Benchmarks Included:
- `blocks_without_votes` - Filter vote transactions from blocks
- `transactions_by_programid_without_votes` - Filter by program ID
- `transactions_by_programid_and_account` - Filter by program ID and account
- `transaction_program_and_account_keys` - Extract keys from single transaction
- `program_ids_without_votes` - Extract program IDs from block
- `program_ids_and_accounts_without_votes` - Extract all keys from block
- `walk_instructions_single_transaction` - Count instructions in one transaction
- `walk_all_instructions_in_block` - Count all instructions in block
- `transaction_retain_with_filter` - Retain operation with filtering
- `block_clone` - Block cloning performance
- `transaction_count` - Transaction count baseline

#### Output:
- Terminal: Statistics (mean, median, std dev)
- HTML: `target/criterion/report/index.html`

---

### 2. Memory Profiling (memory_benchmarks)
**Focus:** Heap allocations and memory usage patterns
**File:** `benches/memory_benchmarks.rs`

#### Run Command:
\`\`\`bash
cargo bench --bench memory_benchmarks
\`\`\`

#### Benchmarks Included:
1. Block cloning memory usage
2. Blocks without votes memory usage
3. Transaction filtering by program ID
4. Key extraction from all transactions
5. Walking all instructions
6. Program ID extraction

#### Output:
- Terminal: Allocation statistics per operation
- File: `dhat-heap.json` (view at https://nnethercote.github.io/dh_view/dh_view.html)

#### Example Output:
\`\`\`
1. Block cloning:
  Block clone
    Total allocations: 1250
    Total bytes allocated: 2458624 (2400.00 KB)
    Current live blocks: 0
    Current live bytes: 0 (0.00 KB)
\`\`\`

---

### 3. Criterion with Allocation Counter (criterion_with_alloc_counter)
**Focus:** Combined time + allocation metrics
**File:** `benches/criterion_with_alloc_counter.rs`

#### Run Command:
\`\`\`bash
cargo bench --bench criterion_with_alloc_counter
\`\`\`

#### Benchmarks Included:
- `block_clone` - With allocation stats
- `blocks_without_votes` - With allocation stats
- `key_extraction` - With allocation stats
- `walk_instructions` - With allocation stats

#### Output:
- Terminal: Criterion timing + allocation statistics
- HTML: Standard Criterion reports

#### Example Output:
\`\`\`
block_clone stats (avg per iteration for 100 iters):
  Bytes allocated: 24586
  Bytes deallocated: 24586
  Allocations: 125
  Deallocations: 125

time:   [1.2345 ms 1.2567 ms 1.2789 ms]
\`\`\`

---

## Quick Reference

| Goal | Command | Output Location |
|------|---------|-----------------|
| Measure execution time | `cargo bench --bench solana_common_benchmarks` | `target/criterion/` |
| Profile memory allocations | `cargo bench --bench memory_benchmarks` | Terminal + `dhat-heap.json` |
| Combined time + allocations | `cargo bench --bench criterion_with_alloc_counter` | Terminal + `target/criterion/` |
| Run all benchmarks | `cargo bench` | Various |
| Quick check (no stats) | `cargo bench -- --quick` | Terminal only |

## Use Cases

### During Development
\`\`\`bash
# Quick performance check
cargo bench --bench solana_common_benchmarks -- blocks_without_votes

# Check memory impact of change
cargo bench --bench memory_benchmarks
\`\`\`

### Before Committing
\`\`\`bash
# Full benchmark suite
cargo bench

# Review HTML reports
open target/criterion/report/index.html
\`\`\`

### Performance Investigation
\`\`\`bash
# Detailed memory profiling
cargo bench --bench memory_benchmarks

# Upload dhat-heap.json to viewer for analysis
\`\`\`

### CI/CD Pipeline
\`\`\`bash
# Quick benchmark without full statistical analysis
cargo bench -- --quick

# Or run with specific timeout
cargo bench --bench solana_common_benchmarks -- --profile-time=5
\`\`\`

## Test Data

All benchmarks use real Solana mainnet block data:
- **File:** `src/testdata/solana_mainnet_313000000.binpb.base64`
- **Block height:** 313,000,000
- **Ensures:** Realistic performance characteristics

## Documentation

- **BENCHMARKS.md** - Detailed benchmark documentation
- **MEMORY_PROFILING.md** - Memory profiling guide
- **This file** - Quick reference summary

## Dependencies

\`\`\`toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
dhat = "0.3"
\`\`\`
