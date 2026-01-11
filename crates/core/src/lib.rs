pub struct Shard {
    pub id: u16, // also likely the core-id on which it runs?
    block_store: BlockStore,
}

pub struct BlockStore {
    hash_map: hashbrown::HashMap<u16, Vec<u16>>,
}

impl BlockStore {
    pub fn new() -> Self {
        Self {
            hash_map: hashbrown::HashMap::new(),
        }
    }
}

impl Shard {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            block_store: BlockStore::new(),
        }
    }

    pub fn get(&self, key: &[u16]) -> Option<&Vec<u16>> {
        self.block_store.hash_map.get(&key[0])
    }
}
