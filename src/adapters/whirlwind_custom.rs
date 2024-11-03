// // WhirlwindShardedMapTable
//
// use std::hash::{BuildHasher, Hash};
//
// use bustle::{Collection, CollectionHandle};
//
// use super::Value;
//
// #[derive(Clone)]
// pub struct WhirlwindCustomMapTable<K: Eq + Hash, H>(whirlwind::table::MyHashMap<K, Value, H>);
//
// impl<K, H> Collection for WhirlwindCustomMapTable<K, H>
// where
//     K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
//     H: BuildHasher + Default + Send + Sync + 'static + Clone,
// {
//     type Handle = Self;
//
//     fn with_capacity(capacity: usize) -> Self {
//         Self(whirlwind::table::MyHashMap::with_capacity_and_hasher(
//             capacity,
//             H::default(),
//         ))
//     }
//
//     fn pin(&self) -> Self::Handle {
//         self.clone()
//     }
// }
//
// impl<K, H> CollectionHandle for WhirlwindCustomMapTable<K, H>
// where
//     K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
//     H: BuildHasher + Default + Send + Sync + 'static + Clone,
// {
//     type Key = K;
//
//     // async fn get(&mut self, key: &Self::Key) -> bool {
//     //     block_on(async { self.0.get(key).await.map(|_| ()) }).is_some()
//     // }
//     //
//     // fn insert(&mut self, key: &Self::Key) -> bool {
//     //     block_on(async { self.0.insert(*key, 0).await.map(|_| ()) }).is_none()
//     // }
//     //
//     // fn remove(&mut self, key: &Self::Key) -> bool {
//     //     block_on(async { self.0.remove(key).await.map(|_| ()) }).is_some()
//     // }
//     //
//     // fn update(&mut self, key: &Self::Key) -> bool {
//     //     block_on(async { self.0.get_mut(key).await.map(|mut v| *v += 1) }).is_some()
//     // }
//
//     async fn get(&mut self, key: &Self::Key) -> bool {
//         self.0.get(key).await.is_some()
//     }
//
//     async fn insert(&mut self, key: &Self::Key) -> bool {
//         self.0.insert(*key, 0).await.is_none()
//     }
//
//     async fn remove(&mut self, key: &Self::Key) -> bool {
//         self.0.remove(key).await.is_some()
//     }
//
//     async fn update(&mut self, key: &Self::Key) -> bool {
//         self.0.get_mut(key).await.map(|v| *v.value += 1).is_some()
//     }
// }
