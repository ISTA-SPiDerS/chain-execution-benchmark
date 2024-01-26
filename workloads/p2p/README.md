# P2P Benchmark

The Peer-to-Peer Benchmark simulates a unique distribution of users transacting with eachother.
The data is based on the peer-to-peer transaction behaviour on the Ethereum blockchain over the course of 2022 and represents the average resource access distribution every 1000 block

In the p2p_workload.rs file there are two vectors describing the distribution of senders and receivers.

The used smart contract can be found in the p2p_contract.move file.
In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
A user transaction touches two keys, the sender balance and the receiver balance.
The number of times we loop was obtained by comparing the transaction run-time to the typical transaction runtime of peer to peer transaction on aptos.


## Rust coding example:

The distribution of resource accesses can be plugged into a weighted distribution in rust as follows:

    let weighted_resource_distribution: WeightedIndex<f64> = WeightedIndex::new(DISTRIBUTION).unwrap();

and random samples can then be obtained through:

    let resource = weighted_resource_distribution.sample(&mut rng)

Specifically, for each transaction we obtain one sender from the sender distribution and one receiver from the receiver distribution.
