# Uniswap Benchmark

The Uniswap Benchmark simulates a unique distribution of users buying assets at an exchange.
The data is based on the uniswap transaction behaviour on the Ethereum blockchain over the course of 2022 and represents the daily average resource access distribution.

In the uniswap_workload.rs file there are two vectors describing the workload.
First an average workload that represents the average distribution of coin pairs over the year.
And second, a bursty variant taking the average of the 30 most bursty days.

The used smart contract can be found in the uniswap_contract.move file.
In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
A user transaction touches one key, the respective coin-pairing.

Each transaction checks a total of eight times in a loop if the key exists in the table (adds it if not) and then increments the value by 1.
The number of times we loop was obtained by comparing the transaction run-time to the typical transaction runtime of peer to peer transaction on aptos.


## Rust coding example:

The distribution of resource accesses can be plugged into a weighted distribution in rust as follows:

    let weighted_resource_distribution: WeightedIndex<f64> = WeightedIndex::new(DISTRIBUTION).unwrap();

and random samples can then be obtained through:

    let resource = weighted_resource_distribution.sample(&mut rng)
