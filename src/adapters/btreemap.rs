use std::collections::BTreeMap;
use std::sync::Arc;

use bustle::*;
use parking_lot::RwLock;
use std::sync::RwLock as StdRwLock;

use super::Value;

#[derive(Clone)]
pub struct ParkingLotRwLockBTreeMapTable<K>(Arc<RwLock<BTreeMap<K, Value>>>);

impl<K> Collection for ParkingLotRwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Handle = Self;

    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(RwLock::new(BTreeMap::new())))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for ParkingLotRwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().get(key).is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().insert(*key, 0).is_none()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().remove(key).is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.write();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}

#[derive(Clone)]
pub struct StdRwLockBTreeMapTable<K>(Arc<StdRwLock<BTreeMap<K, Value>>>);

impl<K> Collection for StdRwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Handle = Self;

    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(StdRwLock::new(BTreeMap::new())))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for StdRwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(key).is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, 0).is_none()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .write()
            .unwrap()
            .get_mut(key)
            .map(|v| *v += 1)
            .is_some()
    }
}
