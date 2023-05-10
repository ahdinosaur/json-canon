use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub object: BTreeMap<Vec<u8>, Vec<u8>>,
    pub next_key: Vec<u8>,
    pub next_val: Vec<u8>,
    pub complete: bool,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            object: BTreeMap::new(),
            next_key: Vec::new(),
            next_val: Vec::new(),
            complete: false,
        }
    }
}
