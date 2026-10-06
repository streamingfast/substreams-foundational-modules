// Memory allocation benchmarks using DHAT
// Run with: cargo bench --bench memory_benchmarks

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Helper function to filter blocks without votes
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

// Helper function to extract transaction program and account keys
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

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    println!("\n=== Memory Profiling Results ===\n");

    // Benchmark 1: Block cloning
    println!("1. Block cloning:");
    let stats_before = dhat::HeapStats::get();
    let _cloned = block.clone();
    let stats_after = dhat::HeapStats::get();
    print_stats_diff("Block clone", &stats_before, &stats_after);

    // Benchmark 2: Blocks without votes
    println!("\n2. Blocks without votes:");
    let stats_before = dhat::HeapStats::get();
    let _filtered = blocks_without_votes_impl(block.clone());
    let stats_after = dhat::HeapStats::get();
    print_stats_diff("Blocks without votes", &stats_before, &stats_after);

    // Benchmark 3: Transaction filtering by program ID
    println!("\n3. Transaction filtering by program ID:");
    let stats_before = dhat::HeapStats::get();
    let query =
        substreams::sqe::expr_matcher("program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
    let mut transactions = block.transactions.clone();
    transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id())]))
    });
    let stats_after = dhat::HeapStats::get();
    print_stats_diff("Filter by program ID", &stats_before, &stats_after);

    // Benchmark 4: Key extraction from all transactions
    println!("\n4. Key extraction from all transactions:");
    let stats_before = dhat::HeapStats::get();
    let keys: Vec<String> = block
        .transactions()
        .flat_map(transaction_program_and_account_keys)
        .collect();
    let stats_after = dhat::HeapStats::get();
    print_stats_diff(
        &format!("Key extraction ({} keys)", keys.len()),
        &stats_before,
        &stats_after,
    );

    // Benchmark 5: Walking all instructions
    println!("\n5. Walking all instructions:");
    let stats_before = dhat::HeapStats::get();
    let count = block.walk_instructions().count();
    let stats_after = dhat::HeapStats::get();
    print_stats_diff(
        &format!("Walk instructions ({} instructions)", count),
        &stats_before,
        &stats_after,
    );

    // Benchmark 6: Program ID extraction
    println!("\n6. Program ID extraction:");
    let stats_before = dhat::HeapStats::get();
    let program_ids: Vec<String> = block
        .walk_instructions()
        .map(|inst| format!("program:{}", inst.program_id()))
        .collect();
    let stats_after = dhat::HeapStats::get();
    print_stats_diff(
        &format!("Program ID extraction ({} IDs)", program_ids.len()),
        &stats_before,
        &stats_after,
    );

    println!("\n=== Memory profiling complete ===");
    println!("Full heap profile will be written to dhat-heap.json");
    println!("View it at: https://nnethercote.github.io/dh_view/dh_view.html");
}

fn print_stats_diff(label: &str, before: &dhat::HeapStats, after: &dhat::HeapStats) {
    let allocs = after.total_blocks - before.total_blocks;
    let bytes = after.total_bytes - before.total_bytes;
    let curr_blocks = after.curr_blocks - before.curr_blocks;
    let curr_bytes = after.curr_bytes - before.curr_bytes;

    println!("  {}", label);
    println!("    Total allocations: {}", allocs);
    println!(
        "    Total bytes allocated: {} ({:.2} KB)",
        bytes,
        bytes as f64 / 1024.0
    );
    println!("    Current live blocks: {}", curr_blocks);
    println!(
        "    Current live bytes: {} ({:.2} KB)",
        curr_bytes,
        curr_bytes as f64 / 1024.0
    );
}
