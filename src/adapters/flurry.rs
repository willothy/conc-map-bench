use bustle::*;
use seize::Collector;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

const BATCH_SIZE: usize = 2000;

#[derive(Clone)]
pub struct FlurryTable<K: 'static, H: 'static>(Arc<flurry::HashMap<K, Value, H>>);

impl<K, H> Collection for FlurryTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(
            flurry::HashMap::with_capacity_and_hasher(capacity, H::default()).with_collector(
                Collector::new()
                    .epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            ),
        ))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for FlurryTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.pin().get(key).is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.pin().insert(*key, 0).is_none()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.pin().remove(key).is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .pin()
            .compute_if_present(key, |_, v| Some(v + 1))
            .is_some()
    }
}
