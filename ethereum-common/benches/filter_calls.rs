use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ethereum_common::{calls::filtered_calls, pb::sf::substreams::ethereum::v1::Calls};
use substreams::{hex, Hex};
use substreams_ethereum::pb::eth::v2::BigInt;

criterion_group!(filter_calls, criterion_benchmark);
criterion_main!(filter_calls);

// To run, you need to edit `Cargo.toml` to comment out the `name` and `crate-type` under
// `[lib]` section, this is to allow benchmarks here to import the crate as a library.
//
// When uncommented, run:
//
// ```
// cargo bench -- --save-baseline develop
// <Make optimizations on code>
//
// # Compares against the develop baseline
// cargo bench -- --baseline develop
// ```

fn criterion_benchmark(c: &mut Criterion) {
    for (name, density) in [
        ("match_0%", 0),
        ("match_33%", 5),
        ("match_66%", 10),
        ("match_100%", 15),
    ] {
        let mut group = c.benchmark_group(format!("{}/count", name));

        let mut query = match density {
            // Matches nothing
            0 => "call_from:0x0000000000000000000000000000000000000000".to_string(),
            n => FROM_ADDRESSES[0..n]
                .iter()
                .map(|address| format!("call_from:0x{}", Hex::encode(address)))
                .collect::<Vec<String>>()
                .join(" || "),
        };
        let query_ptr = query.as_mut_ptr();
        let query_len = query.len();

        for count in [100, 200, 400, 800, 1600, 3200, 6400, 12800].iter() {
            let calls = create_calls(*count, &FROM_ADDRESSES, &TO_ADDRESSES);
            let mut calls_buffer = prost::Message::encode_to_vec(&calls);
            let calls_ptr = calls_buffer.as_mut_ptr();
            let calls_len = calls_buffer.len();

            group.throughput(Throughput::Elements(*count as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(count),
                count,
                |b, &_call_count| {
                    b.iter(|| filtered_calls(query_ptr, query_len, calls_ptr, calls_len));
                },
            );
        }
    }
}

fn create_calls(n: usize, from_addresses: &[[u8; 20]], to_addresses: &[[u8; 20]]) -> Calls {
    let mut calls = Calls {
        calls: Vec::with_capacity(n),
        clock: Some(ethereum_common::pb::sf::substreams::v1::Clock {
            timestamp: Some(prost_types::Timestamp::default()),
            id: "0x".to_string(),
            number: 0,
        }),
    };

    for _ in 0..n {
        calls
            .calls
            .push(ethereum_common::pb::sf::substreams::ethereum::v1::Call {
                call: Some(::substreams_ethereum::pb::eth::v2::Call {
                    caller: from_addresses[n % from_addresses.len()].to_vec(),
                    address: to_addresses[n % to_addresses.len()].to_vec(),
                    input: hex!("").to_vec(),
                    logs: logs(3),
                    balance_changes: balance_changes(5),
                    gas_changes: gas_changes(15),
                    ..Default::default()
                }),
                tx_hash: "0x".to_string(),
            });
    }

    calls
}

fn logs(n: usize) -> Vec<::substreams_ethereum::pb::eth::v2::Log> {
    let mut logs = Vec::with_capacity(n);

    for _ in 0..n {
        logs.push(::substreams_ethereum::pb::eth::v2::Log {
            address: hex!("720cd16b011b987da3518fbf38c3071d4f0d1495").to_vec(),
            topics: vec![
                hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").to_vec(),
            ],
            data: hex!("000000000000000000000000000000000000000000000000000000004b9f96b0").to_vec(),
            ..Default::default()
        });
    }

    logs
}

fn gas_changes(n: usize) -> Vec<::substreams_ethereum::pb::eth::v2::GasChange> {
    let mut gas_changes = Vec::with_capacity(n);

    for _ in 0..n {
        gas_changes.push(::substreams_ethereum::pb::eth::v2::GasChange {
            old_value: 10291212,
            new_value: 291212,
            reason: 10,
            ordinal: 0,
        });
    }

    gas_changes
}

fn balance_changes(n: usize) -> Vec<::substreams_ethereum::pb::eth::v2::BalanceChange> {
    let mut balance_changes = Vec::with_capacity(n);

    for _ in 0..n {
        balance_changes.push(::substreams_ethereum::pb::eth::v2::BalanceChange {
            address: hex!("720cd16b011b987da3518fbf38c3071d4f0d1495").to_vec(),
            old_value: Some(BigInt {
                bytes: hex!("0de0b6b3a7640000").to_vec(),
            }),
            new_value: Some(BigInt {
                bytes: hex!("12b3a7640000").to_vec(),
            }),
            ..Default::default()
        });
    }

    balance_changes
}

static FROM_ADDRESSES: [[u8; 20]; 15] = [
    hex!("01ffffffff111111111111111111111111111111"),
    hex!("02ffffffff111111111111111111111111111111"),
    hex!("03ffffffff111111111111111111111111111111"),
    hex!("04ffffffff111111111111111111111111111111"),
    hex!("05ffffffff111111111111111111111111111111"),
    hex!("06ffffffff111111111111111111111111111111"),
    hex!("07ffffffff111111111111111111111111111111"),
    hex!("08ffffffff111111111111111111111111111111"),
    hex!("09ffffffff111111111111111111111111111111"),
    hex!("10ffffffff111111111111111111111111111111"),
    hex!("11ffffffff111111111111111111111111111111"),
    hex!("12ffffffff111111111111111111111111111111"),
    hex!("13ffffffff111111111111111111111111111111"),
    hex!("14ffffffff111111111111111111111111111111"),
    hex!("15ffffffff111111111111111111111111111111"),
];

static TO_ADDRESSES: [[u8; 20]; 15] = [
    hex!("01ffffffff222222222222222222222222222222"),
    hex!("02ffffffff222222222222222222222222222222"),
    hex!("03ffffffff222222222222222222222222222222"),
    hex!("04ffffffff222222222222222222222222222222"),
    hex!("05ffffffff222222222222222222222222222222"),
    hex!("06ffffffff222222222222222222222222222222"),
    hex!("07ffffffff222222222222222222222222222222"),
    hex!("08ffffffff222222222222222222222222222222"),
    hex!("09ffffffff222222222222222222222222222222"),
    hex!("10ffffffff222222222222222222222222222222"),
    hex!("11ffffffff222222222222222222222222222222"),
    hex!("12ffffffff222222222222222222222222222222"),
    hex!("13ffffffff222222222222222222222222222222"),
    hex!("14ffffffff222222222222222222222222222222"),
    hex!("15ffffffff222222222222222222222222222222"),
];
