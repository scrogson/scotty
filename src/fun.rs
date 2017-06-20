pub mod fun;

use std::collections::HashMap;

pub struct Fun {
}

pub struct Table {
    entries: HashMap<String, Fun>
}

impl Table {
    fn new() -> Table {
        Table {
            entries: HashMap::new()
        }
    }
}
