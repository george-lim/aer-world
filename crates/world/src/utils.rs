use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

use hi_sparse_bitset::{config::_128bit, iter::CachingIndexIter, ops::And, reduce, BitSet};

use crate::EntityId;

#[derive(Default)]
#[repr(transparent)]
pub struct EntitySet(BitSet<_128bit>);

impl EntitySet {
    pub fn intersection(sets: &[Option<&Self>]) -> Self {
        let bitsets = sets.iter().filter_map(|set| set.map(|set| &set.0));
        Self(BitSet::from_iter(reduce(And, bitsets).unwrap()))
    }

    pub fn insert(&mut self, entity: &EntityId) {
        self.0.insert(entity.0)
    }

    pub fn remove(&mut self, entity: &EntityId) -> bool {
        self.0.remove(entity.0)
    }

    pub fn iter(&self) -> EntitySetIter {
        EntitySetIter(self.0.iter())
    }
}

impl<const N: usize> From<[EntityId; N]> for EntitySet {
    fn from(value: [EntityId; N]) -> Self {
        Self(BitSet::from_iter(value.into_iter().map(|entity| entity.0)))
    }
}

#[repr(transparent)]
pub struct EntitySetIter<'a>(CachingIndexIter<&'a BitSet<_128bit>>);

impl<'a> Iterator for EntitySetIter<'a> {
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(EntityId)
    }
}

pub type EntityMap<V> = HashMap<EntityId, V, BuildHasherDefault<EntityHasher>>;

#[derive(Default)]
#[repr(transparent)]
pub struct EntityHasher(usize);

impl Hasher for EntityHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i;
    }
}

#[macro_export]
macro_rules! log_with_indentation {
    ($indentation_level: expr, $fmt: expr) => {
        #[cfg(debug_assertions)]
        {
            print!(
                "{}",
                (0..$indentation_level).map(|_| "\t").collect::<String>()
            );

            println!($fmt)
        }
    };
    ($indentation_level: expr, $fmt: expr, $($args:tt)*) => {
        #[cfg(debug_assertions)]
        {
            print!(
                "{}",
                (0..$indentation_level).map(|_| "\t").collect::<String>()
            );

            println!($fmt, $args)
        }
    };
}
