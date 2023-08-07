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
            i = i + 1;
        };

        move_to(owner, test_tables);
    }

    public entry fun mint(s: &signer, location: address, resource: u64) acquires TestTables {
        let i = 0;
        while (i < 3) {
            i = i + 1;

            let res_table = &mut borrow_global_mut<TestTables>(location).resource_table;
            if (!table::contains(res_table, resource)) {
                table::add(res_table, resource, 0);
            } else {
                let dst_token = table::borrow_mut(res_table, resource);
                *dst_token = *dst_token + 1;
            };
        }
    }
}
