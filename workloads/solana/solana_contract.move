module Owner::benchmark {
    use std::vector;
    use std::table::{Self, Table};

    struct TestTables has key {
        resource_table: Table<u64, u64>
    }

    fun init_module(owner: &signer) {
        let test_tables = TestTables {
            resource_table: table::new()
        };
        let t = &mut test_tables;
        let i = 0;
        while (i < 100) {
            table::add(&mut t.resource_table, i, 1);
            i = i +1;
        };

        move_to(owner, test_tables);
    }

    public entry fun solana_workload(s: &signer, location: address, loop_count: u64, resources: vector<u64>) acquires TestTables {
        let res_table = &mut borrow_global_mut<TestTables>(location).resource_table;

        let i = 0;
        let length = vector::length(&resources);
        while (i < length) {
            let res = *vector::borrow(&resources, i);
            i = i + 1;

            if (!table::contains(res_table, res)) {
                table::add(res_table, res, 0);
            } else {
                let dst_token = table::borrow_mut(res_table, res);
                *dst_token = *dst_token + 1;
            };
        };

        i = 0;
        while (i < loop_count) {
            let j = i % length;
            i = i + 1;

            let res = *vector::borrow(&resources, j);

            if (!table::contains(res_table, res)) {
                table::add(res_table, res, 0);
            } else {
                let dst_token = table::borrow_mut(res_table, res);
                *dst_token = *dst_token + 1;
            };
        };
    }
}
