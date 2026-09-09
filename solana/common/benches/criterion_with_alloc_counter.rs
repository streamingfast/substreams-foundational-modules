// Criterion benchmarks with allocation counting
// This integrates allocation counting into Criterion benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Custom allocator that counts allocations
struct CountingAllocator;

static ALLOCATED: AtomicU64 = AtomicU64::new(0);
static DEALLOCATED: AtomicU64 = AtomicU64::new(0);
static ALLOCATION_COUNT: AtomicU64 = AtomicU64::new(0);
static DEALLOCATION_COUNT: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        DEALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

fn reset_counters() {
    ALLOCATED.store(0, Ordering::Relaxed);
    DEALLOCATED.store(0, Ordering::Relaxed);
    ALLOCATION_COUNT.store(0, Ordering::Relaxed);
    DEALLOCATION_COUNT.store(0, Ordering::Relaxed);
}

fn get_allocation_stats() -> (u64, u64, u64, u64) {
    (
        ALLOCATED.load(Ordering::Relaxed),
        DEALLOCATED.load(Ordering::Relaxed),
        ALLOCATION_COUNT.load(Ordering::Relaxed),
        DEALLOCATION_COUNT.load(Ordering::Relaxed),
    )
}

// Helper functions
fn blocks_without_votes_impl(mut block: Block) -> Block {
    static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

    block.transactions.retain(|trx| {
        if trx.meta.is_unset() || trx.transaction.is_unset() || trx.transaction.message.is_unset() {
            return false;
        }
        if trx.meta.err.is_set() {
            return false;
        }

        let message = &trx.transaction.message;

        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    block
}

fn transaction_program_and_account_keys(
    trx: &ConfirmedTransaction,
) -> impl Iterator<Item = String> + '_ {
    let meta = &trx.meta;
    let message = &trx.transaction.message;

    message
        .account_keys
        .iter()
        .chain(meta.loaded_writable_addresses.iter())
        .chain(meta.loaded_readonly_addresses.iter())
        .map(|acct| format!("account:{}", substreams_solana::base58::encode(acct)))
        .chain(
            trx.walk_instructions()
                .map(|inst| format!("program:{}", inst.program_id())),
        )
}

fn bench_with_allocations(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    let mut group = c.benchmark_group("allocations");

    // Benchmark block cloning
    group.bench_function("block_clone", |b| {
        b.iter_custom(|iters| {
            reset_counters();
            let start = std::time::Instant::now();

            for _i in 0..iters {
                let _cloned = block.clone();
            }

            let elapsed = start.elapsed();
            let (allocated, deallocated, alloc_count, dealloc_count) = get_allocation_stats();

            eprintln!(
                "\nblock_clone stats (avg per iteration for {} iters):",
                iters
            );
            eprintln!("  Bytes allocated: {}", allocated / iters);
            eprintln!("  Bytes deallocated: {}", deallocated / iters);
            eprintln!("  Allocations: {}", alloc_count / iters);
            eprintln!("  Deallocations: {}", dealloc_count / iters);

            elapsed
        });
    });

    // Benchmark blocks without votes
    group.bench_function("blocks_without_votes", |b| {
        b.iter_custom(|iters| {
            reset_counters();
            let start = std::time::Instant::now();

            for _i in 0..iters {
                let _result = blocks_without_votes_impl(block.clone());
            }

            let elapsed = start.elapsed();
            let (allocated, deallocated, alloc_count, dealloc_count) = get_allocation_stats();

            eprintln!(
                "\nblocks_without_votes stats (avg per iteration for {} iters):",
                iters
            );
            eprintln!("  Bytes allocated: {}", allocated / iters);
            eprintln!("  Bytes deallocated: {}", deallocated / iters);
            eprintln!("  Allocations: {}", alloc_count / iters);
            eprintln!("  Deallocations: {}", dealloc_count / iters);

            elapsed
        });
    });

    // Benchmark key extraction
    group.bench_function("key_extraction", |b| {
        b.iter_custom(|iters| {
            reset_counters();
            let start = std::time::Instant::now();

            for _i in 0..iters {
                let _keys: Vec<String> = block
                    .transactions()
                    .flat_map(transaction_program_and_account_keys)
                    .collect();
            }

            let elapsed = start.elapsed();
            let (allocated, deallocated, alloc_count, dealloc_count) = get_allocation_stats();

            eprintln!(
                "\nkey_extraction stats (avg per iteration for {} iters):",
                iters
            );
            eprintln!("  Bytes allocated: {}", allocated / iters);
            eprintln!("  Bytes deallocated: {}", deallocated / iters);
            eprintln!("  Allocations: {}", alloc_count / iters);
            eprintln!("  Deallocations: {}", dealloc_count / iters);

            elapsed
        });
    });

    // Benchmark instruction walking
    group.bench_function("walk_instructions", |b| {
        b.iter_custom(|iters| {
            reset_counters();
            let start = std::time::Instant::now();

            for _i in 0..iters {
                let _count = block.walk_instructions().count();
            }

            let elapsed = start.elapsed();
            let (allocated, deallocated, alloc_count, dealloc_count) = get_allocation_stats();

            eprintln!(
                "\nwalk_instructions stats (avg per iteration for {} iters):",
                iters
            );
            eprintln!("  Bytes allocated: {}", allocated / iters);
            eprintln!("  Bytes deallocated: {}", deallocated / iters);
            eprintln!("  Allocations: {}", alloc_count / iters);
            eprintln!("  Deallocations: {}", dealloc_count / iters);

            elapsed
        });
    });

    group.finish();
}

criterion_group!(benches, bench_with_allocations);
criterion_main!(benches);
