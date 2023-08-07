# P2P Benchmark

The Peer-to-Peer Benchmark simulates a unique distribution of users transacting with eachother.
The data is based on the peer-to-peer transaction behaviour on the Ethereum blockchain over the course of 2022.

In the p2p_workload.rs file there are two sets of tuples describing the unique senders and receivers.
In order to generate the actual probability distributions the tuples have to be unpacked the following way:

    let mut distribution_vector: Vec<f64> = vec![];
    for (key, value) in DISTRIBUTION {
        for i in 0..value {
            distribution_vector.push(key)
        }
    }

This then results in a list of resources where each value represents one resource and its probability to be picked.
I.E. [1,1,1,10] represents 4 resources and their respective probability to be picked, where the first three resources have a probability of 1 (out of 13) and the last resource a probability of 10 (out of 13).

In the case of rust we can then plug the distribution vector into a weighted index.

    let weighted_distribution: WeightedIndex<f64> = WeightedIndex::new(&distribution_vector).unwrap();

We can then query unique values from the index to build a workload that conforms to the probability distribution.

    value = weighted_distribution.sample(&mut rng);

Specifically, for each transaction we obtain one sender from the sender distribution and one receiver from the receiver distribution.

The used smart contract can be found in the p2p_contract.move file.
While this is a Move contract it is straightforward to translate this to different smart contract languages.

In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
A user transaction touches two keys, the sender balance and the receiver balance.

Each transaction checks a total of three times in a loop if both keys exist in the table (adds it if not) and then increments the value by 1 for each.