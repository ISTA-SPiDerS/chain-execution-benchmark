fn create_block(
    num_tx: u64,
    user_account_vector: Vec<Account>,
    load_type: LoadType,
) {
    let mut rng: ThreadRng = thread_rng();

    let mut resource_distribution_data: &[f64] = &AVERAGE_DEX_DISTRIBUTION;
    if matches!(load_type, LoadType::DEXBURSTY)
    {
        resource_distribution_data = &BURSTY_DEX_DISTRIBUTION;
    }
    else if matches!(load_type, LoadType::NFT)
    {
        resource_distribution_data = &NFT_CONTRACT_DISTRIBUTION;
    }
    else if matches!(load_type, LoadType::MIXED)
    {
        resource_distribution_data = &MIXED_DISTRIBUTION;
    }

    let resource_distribution: WeightedIndex<f64> = WeightedIndex::new(resource_distribution_data).unwrap();

    let p2p_receiver_distribution: WeightedIndex<f64> = WeightedIndex::new(&P2P_RECEIVER_DISTRIBUTION).unwrap();
    let p2p_sender_distribution: WeightedIndex<f64> = WeightedIndex::new(&P2P_SENDER_DISTRIBUTION).unwrap();

    let nft_sender_distribution: WeightedIndex<f64> = WeightedIndex::new(&NFT_USER_DISTRIBUTION).unwrap();

    for i in 0..num_tx {
        let mut sender_id: usize = (i as usize) % user_account_vector.len();

        if matches!(load_type, MIXED)
        {
            let cost_sample = GAS_COST_DISTRIBUTION[rand::thread_rng().gen_range(0..GAS_COST_DISTRIBUTION.len())];
            let write_len_sample = WRITE_LENGTH_DISTRIBUTION[rand::thread_rng().gen_range(0..WRITE_LENGTH_DISTRIBUTION.len())] as usize;

            let mut writes: Vec<u64> = Vec::new();
            let mut i = 0;
            while i < write_len_sample {
                i+=1;
                writes.push(resource_distribution.sample(&mut rng) as u64);
            }

            let length = max(1, cost_sample.round() as usize);
            println!("mixed_workload: {}{:?}", length, writes);
        }
        else if matches!(load_type, P2PTX)
        {
            let receiver_id = p2p_receiver_distribution.sample(&mut rng) % user_account_vector.len();
            sender_id = p2p_sender_distribution.sample(&mut rng) % user_account_vector.len();

            println!("p2p_workload: {}{}", receiver_id, sender_id);
        }
        else if matches!(load_type, NFT)
        {
            let resource_id = resource_distribution.sample(&mut rng);
            sender_id = nft_sender_distribution.sample(&mut rng) % user_account_vector.len();

            println!("mint: {}{}", resource_id, sender_id);
        }
        else
        {
            let resource_id = resource_distribution.sample(&mut rng);
            println!("exchange: {}", resource_id);
        }
    }
}