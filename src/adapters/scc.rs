use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;
use bustle::*;
use scc::hash_map::{Entry, HashMap};

#[derive(Clone)]
pub struct SccMapTable<K, H>(Arc<HashMap<K, Value, H>>)
where
    K: Eq + Hash,
    H: BuildHasher;

impl<K, H> Collection for SccMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read(key, |_, v| *v).is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_ok()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        match self.0.entry(*key) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                true
            }
            Entry::Vacant(_) => false,
        }
    }
}
