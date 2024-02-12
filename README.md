# Blockchain Execution Engine Benchmark - BEEB

This benchmark consists of a total of five individual benchmarks:
- NFT minting
- P2P Transactions
- UniSwap Trading (Daily Average)
- UniSwap Trading (Average of the 30 most bursty days)
- Mixed Workload

All workloads can be found in the [workloads](workloads) folder including documentation on how to use the datasets to generate the workload as well as example move smart contract code.
In the [example](example) folder you may find the full example code on how to use the workload vectors to construct the benchmark.

We created the workloads by recording resource accesses on Ethereum and Solana over the course of 2022 and taking an average from the observations.

The resource distribution is provided in the form of a list of resources where each value represents one resource and its probability to be picked.
I.E. [1,1,1,10] represents 4 resources and their respective probability to be picked, where the first three resources have a probability of 1 (out of 13) and the last resource has a probability of 10 (out of 13).

All smart contracts in this repository are currently written using the Move [Move](https://github.com/move-language/move) Smart Contract Language. 
Nonetheless, we kept the smart contract code simple to facilitate porting this benchmark to other smart contract execution engines.
