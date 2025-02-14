use mixed_workload::{MIXED_DISTRIBUTION, WRITE_LENGTH_DISTRIBUTION};
use nft_workload::{NFT_CONTRACT_DISTRIBUTION, NFT_USER_DISTRIBUTION};
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
    seq::IndexedRandom,
    Rng,
};
use uniswap_workload::{AVERAGE_VALUE_DISTRIBUTION, BURSTY_VALUE_DISTRIBUTION};

use crate::p2p_workload::{RECEIVER_DISTRIBUTION, SENDER_DISTRIBUTION};

mod mixed_workload;
mod nft_workload;
mod p2p_workload;
mod uniswap_workload;

/// Returns the percentile of a sorted vector.
fn percentile(sorted_vec: &[f64], percentile: f64) -> Option<f64> {
    let len = sorted_vec.len();
    if len == 0 {
        return None;
    }

    let index = ((percentile / 100.0) * len as f64).ceil() as usize - 1;
    Some(sorted_vec[index.min(len - 1)])
}

/// Solana workload.
/// Generate a transaction with a random number of inputs (ignoring reads).
fn solana_concurrency<R: Rng>(rng: &mut R) {
    let number_of_inputs = WRITE_LENGTH_DISTRIBUTION
        .choose(rng)
        .expect("Empty distribution")
        .round() as usize;

    let dist = WeightedIndex::new(&MIXED_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero");
    let inputs = (0..number_of_inputs)
        .map(|_| dist.sample(rng))
        .collect::<Vec<_>>();

    let median_number_of_inputs =
        percentile(&WRITE_LENGTH_DISTRIBUTION, 50.0).expect("Empty distribution");
    let p70_number_of_inputs =
        percentile(&WRITE_LENGTH_DISTRIBUTION, 70.0).expect("Empty distribution");

    println!("Solana workload");
    println!("Sampled a transaction with input objects: {inputs:?}");
    println!("Median number of inputs: {median_number_of_inputs}");
    println!("P70 number of inputs: {p70_number_of_inputs}\n");
}

/// Ethereum transfers workload.
/// Generate a transfer transaction.
pub fn ethereum_transfers<R: Rng>(rng: &mut R) {
    // Generate a transfer transaction.
    let sender = WeightedIndex::new(&SENDER_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);
    let recipient = WeightedIndex::new(&RECEIVER_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);

    println!("Ethereum transfer");
    println!("Sampled transfer: from {sender} to {recipient}\n");
}

/// Ethereum NFT mint.
/// Generate a typical NFT mint transaction.
fn ethereum_nft_mint<R: Rng>(rng: &mut R) {
    // Generate a typical NFT mint transaction.
    let object_id = WeightedIndex::new(&NFT_CONTRACT_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);
    let minter = WeightedIndex::new(&NFT_USER_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);

    println!("Ethereum NFT mint");
    println!("Sampled mint: user {minter} minted NFT {object_id}\n");
}

/// Ethereum Uniswap workload.
/// Generate a Uniswap transaction during normal operations.
fn ethereum_uniswap_normal<R: Rng>(rng: &mut R) {
    let coin_pair = WeightedIndex::new(AVERAGE_VALUE_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);

    println!("Ethereum Uniswap (normal operations)");
    println!("Uniswap transaction swapped coin pair {coin_pair}\n");
}

/// Ethereum Uniswap workload.
/// Generate a Uniswap transaction during peak time.
fn ethereum_uniswap_peak<R: Rng>(rng: &mut R) {
    let coin_pair = WeightedIndex::new(BURSTY_VALUE_DISTRIBUTION)
        .expect("Weights should be non-negative and not all zero")
        .sample(rng);

    println!("Ethereum Uniswap (peak times)");
    println!("Uniswap transaction swapped coin pair {coin_pair}\n");
}

fn main() {
    let mut rng = rng();

    // Solana workload.
    solana_concurrency(&mut rng);

    // Ethereum transfers workload.
    ethereum_transfers(&mut rng);

    // Ethereum NFT mint.
    ethereum_nft_mint(&mut rng);

    // Ethereum Uniswap workload.
    ethereum_uniswap_normal(&mut rng);

    // Ethereum Uniswap workload.
    ethereum_uniswap_peak(&mut rng);
}
