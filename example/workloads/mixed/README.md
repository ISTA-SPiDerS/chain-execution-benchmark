# Mixed Benchmark

The Mixed Benchmark simulates users accessing several resources in a single transaction with varying transaction runtimes.
The data is based on the real world Solana transactional history on the Solana blockchain over the course of 2022 and represents the average resource access distribution every 1000 blocks.
Unlike in the case of the other workloads, we increase the set of resources by a factor of 10, as each transaction accesses several resources to approximate the pattern of conflicting transactions we observed at Solana.

In the mixed_workload.rs file there are three vectors describing the workload.
First the distribution of resource accesses. 
Second, the distribution of the number of different resource accesses per transacation.
And third, the gas cost distribution per transaction, representing the transaction runtime.

An example for the smart contract can be found in the mixed_contract.move file.
In a nutshell we have one resource table (or map) where conflicts only happen when two transactions access the same key/entry.
A mixed transaction touches different keys several times depending on the cost and write length.

## Rust coding example:

The distribution of resource accesses could be plugged into a weighted distribution in rust as follows:

    let weighted_resource_distribution: WeightedIndex<f64> = WeightedIndex::new(DISTRIBUTION).unwrap();

and random samples can then be obtained through:

    let resource = weighted_resource_distribution.sample(&mut rng)

For the transaction length and gas cost, each value has the same probability. Thus, we can just pick a value by random.

    let cost_sample = COST_DISTR[rand::thread_rng().gen_range(0..COST_DISTR.len())];
    let write_len_sample = LEN_DISTR[rand::thread_rng().gen_range(0..LEN_DISTR.len())] as usize;

As such, for each transaction, we generate a set of resources of random length and a "cost" factor that defines the transaction runtime.






