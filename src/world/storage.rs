use world::block;

pub struct BlockStorage {
    blocks: Vec<u16>,
}

impl BlockStorage {
    pub fn new(size: usize) -> BlockStorage {
        Self::new_default(size, block::Air{})
    }

    pub fn new_default(size: usize, def: block::Block) -> BlockStorage {
        BlockStorage {
            blocks: vec![def.internal_id(); size]
        }
    }

    #[inline]
    pub fn get(&self, idx: usize) -> block::Block {
        block::Block::by_internal_id(self.blocks[idx])
    }

    #[inline]
    pub fn set(&mut self, idx: usize, b: block::Block) -> bool {
        let old = self.get(idx);
        if old == b {
            false
        } else {
            // NOTE: We can't use vanilla ids since that isn't imlpemented for all types
            self.blocks[idx] = b.internal_id();
            true
        }
    }
}
