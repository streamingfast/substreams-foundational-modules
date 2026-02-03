# Memory Profiling Quick Reference

This guide shows you how to measure memory allocations in the solana-common benchmarks.

## Quick Start - Best Options by Platform

### macOS (your current platform)
```bash
# Option 1: DHAT profiler (easiest and most detailed)
cd solana/common
cargo bench --bench memory_benchmarks

# View the generated dhat-heap.json at:
# https://nnethercote.github.io/dh_view/dh_view.html

# Option 2: Allocation counter (shows stats in terminal)
cargo bench --bench criterion_with_alloc_counter
```

### Linux
All macOS options plus:
```bash
# Valgrind Massif (heap profiler)
valgrind --tool=massif --massif-out-file=massif.out \
  target/release/deps/memory_benchmarks-*

# View results
ms_print massif.out
```

## Method Comparison

| Method | Platform | Detail Level | Ease of Use | Output Format |
|--------|----------|--------------|-------------|---------------|
| **DHAT** | All | Very High | Easy | Interactive HTML |
| **Alloc Counter** | All | Medium | Easy | Terminal stats |
| **Valgrind/Massif** | Linux | High | Medium | Text report |
| **Heaptrack** | Linux | Very High | Medium | GUI |
| **Instruments** | macOS | Very High | Medium | GUI |

## What Each Method Shows

### 1. DHAT (Recommended for Most Cases)

**Run:**
```bash
cargo bench --bench memory_benchmarks
```

**Shows:**
- Total bytes allocated per operation
- Number of allocations
- Peak heap usage
- Allocation lifetimes
- Call stacks for allocations
- Memory leaks

**Output example:**
```
1. Block cloning:
  Block clone
    Total allocations: 1,250
    Total bytes allocated: 2,458,624 (2400.00 KB)
    Current live blocks: 0
    Current live bytes: 0 (0.00 KB)
```

**View detailed results:**
1. Upload `dhat-heap.json` to https://nnethercote.github.io/dh_view/dh_view.html
2. Explore allocation hotspots, lifetimes, and call stacks

### 2. Allocation Counter (Best for Quick Checks)

**Run:**
```bash
cargo bench --bench criterion_with_alloc_counter
```

**Shows:**
- Bytes allocated per iteration
- Number of allocations per iteration
- Integration with Criterion's timing data

**Output example:**
```
block_clone stats (avg per iteration for 100 iters):
  Bytes allocated: 24586
  Bytes deallocated: 24586
  Allocations: 125
  Deallocations: 125

time:   [1.2345 ms 1.2567 ms 1.2789 ms]
```

**Use case:** Quick checks during development, CI/CD pipelines

### 3. Standard Criterion (Baseline Timing)

**Run:**
```bash
cargo bench --bench solana_common_benchmarks
```

**Shows:**
- Execution time statistics
- Performance regression detection
- HTML reports with graphs

**Use case:** Performance optimization without memory focus

## Common Patterns to Look For

### 1. Excessive Allocations
```
Operation: key_extraction
  Allocations: 15,000 per iteration  <-- HIGH
```
**Fix:** Pre-allocate with `Vec::with_capacity()` or use iterators without collecting

### 2. Memory Leaks
```
Current live blocks: 100  <-- LEAK
Current live bytes: 50,000 KB
```
**Fix:** Check for `Rc` cycles, ensure `Drop` implementations

### 3. Large Clone Operations
```
block_clone
  Bytes allocated: 5,242,880 (5 MB per clone)
```
**Fix:** Use references (`&Block`) or `Arc` for shared ownership

### 4. String Allocations
```
program_ids_extraction
  Allocations: 5,000  <-- Many small allocations
```
**Fix:** Use `&str` where possible, reuse String buffers

## Real-World Example

Let's say you want to optimize the `blocks_without_votes` function:

### Step 1: Baseline with DHAT
```bash
cargo bench --bench memory_benchmarks
```

Output:
```
blocks_without_votes
  Total allocations: 2,500
  Total bytes: 1,048,576 (1024 KB)
```

### Step 2: Identify Hotspots
Upload `dhat-heap.json` to the viewer and see:
- 80% of allocations from `retain()` operation
- Multiple Vec reallocations

### Step 3: Optimize
```rust
// Before: repeated reallocations
block.transactions.retain(|trx| { ... });

// After: pre-calculate size
let filtered: Vec<_> = block.transactions
    .into_iter()
    .filter(|trx| { ... })
    .collect();
block.transactions = filtered;
```

### Step 4: Verify
```bash
cargo bench --bench memory_benchmarks
```

New output:
```
blocks_without_votes
  Total allocations: 800  (-68%)
  Total bytes: 524,288 (-50%)
```

## Tips

1. **Run multiple times**: Memory measurements can vary, run 3-5 times and compare
2. **Watch for regressions**: Save `dhat-heap.json` files and compare before/after optimizations
3. **Profile in release mode**: Benchmarks automatically use release mode with optimizations
4. **Focus on hot paths**: Optimize the operations called most frequently first
5. **Consider trade-offs**: Sometimes more allocations with smaller size is better than fewer large allocations

## Troubleshooting

### "dhat-heap.json not generated"
The file is created in the current directory where you run the benchmark. Check:
```bash
ls -la dhat-heap.json
```

### "Allocation numbers seem wrong"
The allocator counts all allocations including test infrastructure. Focus on relative differences between operations, not absolute numbers.

### "Can't open dhat viewer"
The DHAT viewer runs entirely in your browser. Just open the URL and drag/drop the JSON file - no upload to external servers.

## Further Reading

- [DHAT Documentation](https://valgrind.org/docs/manual/dh-manual.html)
- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Allocation Patterns in Rust](https://nnethercote.github.io/perf-book/heap-allocations.html)
