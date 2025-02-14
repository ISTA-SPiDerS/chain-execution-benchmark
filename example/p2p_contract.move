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

    public entry fun p2p_workload(s: &signer, location: address, resource1: u64, resource2: u64) acquires TestTables {
        let i = 0;
        while (i < 8) {
            i = i + 1;

            let res_table = &mut borrow_global_mut<TestTables>(location).resource_table;
            if (!table::contains(res_table, resource1)) {
                table::add(res_table, resource1, 0);
            } else {
                let dst_token = table::borrow_mut(res_table, resource1);
                *dst_token = *dst_token + 1;
            };

            if (!table::contains(res_table, resource2)) {
                table::add(res_table, resource2, 0);
            } else {
                let dst_token = table::borrow_mut(res_table, resource2);
                *dst_token = *dst_token + 1;
            };
        }
    }
}
