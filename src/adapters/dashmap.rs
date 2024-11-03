use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;
use dashmap::DashMap;

use super::Value;

#[derive(Clone)]
pub struct DashMapTable<K, H>(Arc<DashMap<K, Value, H>>);

impl<K, H> Collection for DashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(DashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for DashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get_mut(key).map(|mut v| *v += 1).is_some()
    }
}
