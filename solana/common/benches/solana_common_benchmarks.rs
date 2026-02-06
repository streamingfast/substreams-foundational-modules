use criterion::{black_box, criterion_group, criterion_main, Criterion};
use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

// Include the library modules directly for benchmarking
// This is necessary because cargo bench treats the library as an external crate

// Helper function to filter blocks without votes (mirrors block.rs)
fn blocks_without_votes_impl(mut block: Block) -> Block {
    static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

    block.transactions.retain(|trx| {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };
        if meta.err.is_some() {
            return false;
        }

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => return false,
        };
        let message = transaction.message.as_ref().expect("Message is missing");

        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    block
}

// Helper function to extract transaction program and account keys (mirrors keys.rs)
fn transaction_program_and_account_keys(
    trx: &ConfirmedTransaction,
) -> impl Iterator<Item = String> + '_ {
    let meta = trx.meta.as_ref().unwrap();
    let message = trx.transaction.as_ref().unwrap().message.as_ref().unwrap();

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

// Benchmark for blocks_without_votes
fn bench_blocks_without_votes(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("blocks_without_votes", |b| {
        b.iter(|| {
            let block_clone = block.clone();
            blocks_without_votes_impl(black_box(block_clone))
        });
    });
}

// Benchmark for transaction filtering by program ID
fn bench_transactions_by_programid(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");
    let query_str = "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

    c.bench_function("transactions_by_programid_without_votes", |b| {
        b.iter(|| {
            let query = substreams::sqe::expr_matcher(black_box(query_str));
            let mut transactions = block.transactions.clone();

            transactions.retain(|trx| {
                trx.walk_instructions()
                    .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id())]))
            });

            black_box(transactions)
        });
    });
}

// Benchmark for transaction filtering by program ID and account
fn bench_transactions_by_programid_and_account(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");
    let query_str = "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc && account:5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc";

    c.bench_function("transactions_by_programid_and_account", |b| {
        b.iter(|| {
            let query = substreams::sqe::expr_matcher(black_box(query_str));
            let mut transactions = block.transactions.clone();

            transactions.retain(|trx| {
                let keys: Vec<_> = transaction_program_and_account_keys(trx).collect();
                query.matches_keys(&keys)
            });

            black_box(transactions)
        });
    });
}

// Benchmark for key extraction from transactions
fn bench_transaction_program_and_account_keys(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");
    let transaction = block.transactions.get(8).unwrap();

    c.bench_function("transaction_program_and_account_keys", |b| {
        b.iter(|| {
            let keys: Vec<String> =
                transaction_program_and_account_keys(black_box(transaction)).collect();
            black_box(keys)
        });
    });
}

// Benchmark for extracting program IDs from a block
fn bench_program_ids_extraction(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("program_ids_without_votes", |b| {
        b.iter(|| {
            let block_clone = block.clone();
            let keys: Vec<String> = black_box(block_clone)
                .walk_instructions()
                .map(|inst| format!("program:{}", inst.program_id()))
                .collect();
            black_box(keys)
        });
    });
}

// Benchmark for extracting program IDs and accounts from a block
fn bench_program_ids_and_accounts_extraction(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("program_ids_and_accounts_without_votes", |b| {
        b.iter(|| {
            let block_clone = block.clone();
            let keys: Vec<String> = black_box(block_clone)
                .transactions()
                .flat_map(transaction_program_and_account_keys)
                .collect();
            black_box(keys)
        });
    });
}

// Benchmark for instruction walking on a single transaction
fn bench_walk_instructions(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");
    let transaction = block.transactions.get(8).unwrap();

    c.bench_function("walk_instructions_single_transaction", |b| {
        b.iter(|| {
            let count = black_box(transaction).walk_instructions().count();
            black_box(count)
        });
    });
}

// Benchmark for full block processing (walk all instructions)
fn bench_walk_all_instructions_in_block(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("walk_all_instructions_in_block", |b| {
        b.iter(|| {
            let block_clone = block.clone();
            let count = black_box(block_clone).walk_instructions().count();
            black_box(count)
        });
    });
}

// Benchmark for transaction retain operation (common pattern)
fn bench_transaction_retain_operation(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("transaction_retain_with_filter", |b| {
        b.iter(|| {
            let mut transactions = block.transactions.clone();

            transactions.retain(|trx| {
                black_box(trx).walk_instructions().any(|view| {
                    view.program_id().to_string() == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
                })
            });

            black_box(transactions)
        });
    });
}

// Benchmark for block cloning operation
fn bench_block_clone(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("block_clone", |b| {
        b.iter(|| {
            let cloned = black_box(&block).clone();
            black_box(cloned)
        });
    });
}

// Benchmark for transaction count in block
fn bench_transaction_count(c: &mut Criterion) {
    let block: Block = testing::read_block("./src/testdata/solana_mainnet_313000000.binpb.base64");

    c.bench_function("transaction_count", |b| {
        b.iter(|| {
            let count = black_box(&block).transactions.len();
            black_box(count)
        });
    });
}

criterion_group!(
    benches,
    bench_blocks_without_votes,
    bench_transactions_by_programid,
    bench_transactions_by_programid_and_account,
    bench_transaction_program_and_account_keys,
    bench_program_ids_extraction,
    bench_program_ids_and_accounts_extraction,
    bench_walk_instructions,
    bench_walk_all_instructions_in_block,
    bench_transaction_retain_operation,
    bench_block_clone,
    bench_transaction_count,
);

criterion_main!(benches);
