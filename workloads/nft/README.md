# NFT Benchmark

The NFT Benchmark simulates a unique distribution of users minting NFTs from several smart contracts.
The data is based on the minting behaviour on the Ethereum blockchain over the course of 2022.

In the nft_workload.rs file there are two vectors describing the workload.
For once the distribution of different NFT smart contracts, and on the other hand, the distribution of users accessing the contracts.

The used smart contract can be found in the nft_contract.move file.
In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
Each transaction checks a total of eight times in a loop if the key exists in the table (adds it if not) and then increments the value by 1.
The number of times we loop was obtained by comparing the transaction run-time to the typical transaction runtime of peer to peer transaction on aptos.


## Rust coding example:

The distribution of resource accesses can be plugged into a weighted distribution in rust as follows:

    let weighted_resource_distribution: WeightedIndex<f64> = WeightedIndex::new(DISTRIBUTION).unwrap();

and random samples can then be obtained through:

    let resource = weighted_resource_distribution.sample(&mut rng)

As such, for each transaction we obtain one sender from the sender distribution and one resource key from the contract distribution.




