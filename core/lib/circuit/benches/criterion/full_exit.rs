use crate::generate_accounts;
use crate::utils::ZkSyncStateGenerator;
use criterion::{criterion_group, Bencher, BenchmarkId, Criterion};
use num::BigUint;
use zksync_circuit::witness::Witness;
use zksync_crypto::franklin_crypto::bellman::pairing::bn256::Bn256;
use zksync_types::{FullExit, FullExitOp};

use zksync_circuit::witness::full_exit::FullExitWitness;

type FullExitWitnessBn256 = FullExitWitness<Bn256>;

/// Measures the time of full exit witness
fn full_exit_witness(b: &mut Bencher<'_>, number_of_accounts: &usize) {
    let accounts = generate_accounts(*number_of_accounts);
    let account = &accounts[0];
    let full_exit_op = FullExitOp {
        priority_op: FullExit {
            account_id: account.id,
            eth_address: account.account.address,
            token: 0,
        },
        withdraw_amount: Some(BigUint::from(10u32).into()),
    };
    let (_, circuit_account_tree) = ZkSyncStateGenerator::generate(&accounts);

    let setup = || (circuit_account_tree.clone());
    b.iter_with_setup(setup, |mut circuit_account_tree| {
        FullExitWitnessBn256::apply_tx(&mut circuit_account_tree, &(full_exit_op.clone(), true));
    });
}

pub fn bench_full_exit(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("Full exit witness", 1usize),
        &1usize,
        full_exit_witness,
    );
    c.bench_with_input(
        BenchmarkId::new("Full exit witness", 10usize),
        &10usize,
        full_exit_witness,
    );
    c.bench_with_input(
        BenchmarkId::new("Full exit witness", 100usize),
        &100usize,
        full_exit_witness,
    );
}

criterion_group!(full_exit_benches, bench_full_exit);
