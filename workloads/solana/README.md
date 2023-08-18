# Solana Benchmark

The Solana Benchmark simulates users accessing several resources within the same transaction with varying gas costs.
The data is based on the actual solana transaction behaviour on the Solana blockchain over the course of 2022.

In the solana_workload.rs file there are three sets of tuples describing the workload.
First the probability distribution of resource accesses. 
Second, the distribution of the number of different resource accesses per transacation.
And third, the gas cost distribution per transaction.

In order to generate the actual probability distribution the vector has to be unpacked the following way.

    let mut distribution_vector: Vec<f64> = vec![];
    for value in X_DISTRIBUTION {
        for i in 0..20 {
            distribution_vector.push(value)
        }
    }

Unlike in the case of the other workloads, we increase the set of resources by a factor of 20, as each transaction accesses several resources.
We executed the resulting workload several times with different configurations to find a configuration that approximates the path pattern we observed at Solana.

This then results in a list of resources where each value represents one resource and its probability to be picked.
I.E. [1,1,1,10] represents 4 resources and their respective probability to be picked, where the first three resources have a probability of 1 (out of 13) and the last resource a probability of 10 (out of 13).

In the case of rust we can then plug the distribution vector into a weighted index.

    let weighted_distribution: WeightedIndex<f64> = WeightedIndex::new(&distribution_vector).unwrap();

We can then query unique values from the index to build a workload that conforms to the probability distribution.

    value = weighted_distribution.sample(&mut rng);


For the transaction length and gas cost vectors present the average values we observed in the evaluation period. As such, we can just pick random values from these vectors directly.

The used smart contract can be found in the solana_contract.move file.
While this is a Move contract it is straightforward to translate this to different smart contract languages.

In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
A user transaction touches several keys.

Depending on the gas cost, each user transaction iterates several time over the access and incrementation logic.



