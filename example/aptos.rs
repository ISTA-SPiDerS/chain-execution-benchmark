fn create_block(
    num_tx: u64,
    contract_owner: AccountData,
    account_vector: Vec<Account>,
    seq_num: &mut HashMap<usize, u64>,
    contract_id: &ModuleId,
    load_type: LoadType,
) -> VecDeque<SignedTransaction> {

    let mut result = VecDeque::new();
    let mut rng: ThreadRng = thread_rng();

    let mut resource_distribution_vec:Vec<f64> = vec![];
    if matches!(load_type, LoadType::DEXAVG)
    {
        for (key, value) in AVERAGE_VALUE_DISTRIBUTION {
            for i in 0..value {
                resource_distribution_vec.push(key)
            }
        }
        println!("{}", resource_distribution_vec.len())
    }
    else if matches!(load_type, LoadType::DEXBURSTY)
    {
        for (key, value) in BURSTY_VALUE_DISTRIBUTION {
            for i in 0..value {
                resource_distribution_vec.push(key)
            }
        }
    }
    else if matches!(load_type, LoadType::NFT)
    {
        for (key, value) in NFT_DISTRIBUTION {
            for i in 0..value {
                resource_distribution_vec.push(key)
            }
        }
    }
    else if matches!(load_type, LoadType::SOLANA)
    {
        for (key, value) in VALUE_DISTRIBUTION {
            for i in 0..value * 20 {
                resource_distribution_vec.push(key)
            }
        }
    }

    let mut solana_len_options:Vec<usize> = vec![];
    let mut solana_len_distr_vec:Vec<u64> = vec![];

    for (key, value) in WRITE_LENGTH_DISTRIBUTION {
        solana_len_options.push(key.round() as usize);
        solana_len_distr_vec.push(value);
    }

    let mut solana_cost_options:Vec<f64> = vec![];
    let mut solana_cost_distr_vec:Vec<u64> = vec![];

    for (key, value) in GAS_COST_DISTRIBUTION {
        solana_cost_options.push(key);
        solana_cost_distr_vec.push(value);
    }

    let solana_len_distribution: WeightedIndex<u64> = WeightedIndex::new(&solana_len_distr_vec).unwrap();
    let solana_cost_distribution: WeightedIndex<u64> = WeightedIndex::new(&solana_cost_distr_vec).unwrap();

    let general_resource_distribution: WeightedIndex<f64> = WeightedIndex::new(&resource_distribution_vec).unwrap();

    let mut nft_sender_distr_vec: Vec<f64> = vec![];
    for (key, value) in USER_DISTRIBUTION {
        for i in 0..value {
            nft_sender_distr_vec.push(key)
        }
    }
    let nft_sender_distribution: WeightedIndex<f64> = WeightedIndex::new(&nft_sender_distr_vec).unwrap();

    let mut p2p_sender_distr_vec:Vec<f64> = vec![];
    let mut p2p_receiver_distr_vec:Vec<f64> = vec![];

    for (key, value) in RECEIVER_DISTRIBUTION {
        for i in 0..value {
            p2p_receiver_distr_vec.push(key);
        }
    }

    for (key, value) in SENDER_DISTRIBUTION {
        for i in 0..value {
            p2p_sender_distr_vec.push(key);
        }
    }

    let p2p_receiver_distribution: WeightedIndex<f64> = WeightedIndex::new(&p2p_receiver_distr_vec).unwrap();
    let p2p_sender_distribution: WeightedIndex<f64> = WeightedIndex::new(&p2p_sender_distr_vec).unwrap();

    for i in 0..num_tx {
        let mut sender_id: usize = (i as usize) % account_vector.len();
        let tx_entry_function;

        if matches!(load_type, SOLANA)
        {
            let cost_sample = solana_cost_options[solana_cost_distribution.sample(&mut rng)];
            let write_len_sample = solana_len_options[solana_len_distribution.sample(&mut rng)];

            let mut writes: Vec<u64> = Vec::new();
            let mut i = 0;
            while i < write_len_sample {
                i+=1;
                writes.push(general_resource_distribution.sample(&mut rng) as u64);
            }

            let length = max(1, cost_sample.round() as usize);

            tx_entry_function = EntryFunction::new(
                contract_id.clone(),
                ident_str!("solana_workload").to_owned(),
                vec![],
                vec![bcs::to_bytes(contract_owner.address()).unwrap(), bcs::to_bytes(&length).unwrap(), bcs::to_bytes(&writes).unwrap()],
            );
        }
        else if matches!(load_type, P2PTX)
        {
            let receiver_id = p2p_receiver_distribution.sample(&mut rng) % account_vector.len();
            let sender_id = p2p_sender_distribution.sample(&mut rng) % account_vector.len();

            tx_entry_function = EntryFunction::new(
                contract_id.clone(),
                ident_str!("p2p_workload").to_owned(),
                vec![],
                vec![bcs::to_bytes(contract_owner.address()).unwrap(), bcs::to_bytes(&receiver_id).unwrap(), bcs::to_bytes(&sender_id).unwrap()],
            );
        }
        else if matches!(load_type, NFT)
        {
            resource_id = general_resource_distribution.sample(&mut rng);
            sender_id = nft_sender_distribution.sample(&mut rng) % account_vector.len();

            tx_entry_function = EntryFunction::new(
                contract_id.clone(),
                ident_str!("mint").to_owned(),
                vec![],
                vec![bcs::to_bytes(contract_owner.address()).unwrap(), bcs::to_bytes(&resource_id).unwrap()],
            );
        }
        else
        {
            resource_id = general_resource_distribution.sample(&mut rng);

            tx_entry_function = EntryFunction::new(
                contract_id.clone(),
                ident_str!("exchange").to_owned(),
                vec![],
                vec![bcs::to_bytes(contract_owner.address()).unwrap(), bcs::to_bytes(&resource_id).unwrap()],
            );
        }

        let txn = account_vector[sender_id]
            .transaction()
            .entry_function(tx_entry_function.clone())
            .sequence_number(seq_num[&sender_id])
            .sign();
        seq_num.insert(sender_id, seq_num[&sender_id] + 1);
        result.push_back(txn);
    }
    // println!("{:?}", result);

    result
}