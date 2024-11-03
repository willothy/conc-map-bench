use std::{future::Future, task::Poll};

use std::hash::{BuildHasher, Hash, RandomState};
use std::sync::Arc;

use bustle::*;

use super::Value;

pub trait DashMapAsync<'a, K, V>
where
    K: std::hash::Hash + Eq + Clone + 'a,
    V: 'a,
{
    #[allow(unused)]
    fn entry_async(
        &'a self,
        key: K,
    ) -> impl Future<Output = dashmap::mapref::entry::Entry<'a, K, V>>;

    fn get_async<'b: 'a>(
        &'a self,
        key: &'b K,
    ) -> impl Future<Output = Option<dashmap::mapref::one::Ref<'a, K, V>>>;

    fn get_mut_async<'b: 'a>(
        &'a self,
        key: &'b K,
    ) -> impl Future<Output = Option<dashmap::mapref::one::RefMut<'a, K, V>>>;

    fn insert_async(&'a self, key: K, value: V) -> impl Future<Output = Option<V>>;

    fn remove_async(&'a self, key: K) -> impl Future<Output = Option<V>>;
}

pub struct EntryFuture<'a, K, V, S> {
    key: K,
    map: &'a dashmap::DashMap<K, V, S>,
}

impl<'a, K, V, S> Future for EntryFuture<'a, K, V, S>
where
    K: std::hash::Hash + Eq + Clone,
    V: 'static,
    S: BuildHasher + Clone,
{
    type Output = dashmap::mapref::entry::Entry<'a, K, V>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.map.try_entry(self.key.clone()) {
            Some(entry) => Poll::Ready(entry),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub struct GetFuture<'a, 'b, K, V, S> {
    key: &'b K,
    map: &'a dashmap::DashMap<K, V, S>,
}

impl<'a, 'b, K, V, S> Future for GetFuture<'a, 'b, K, V, S>
where
    K: std::hash::Hash + Eq + Clone,
    V: 'static,
    S: BuildHasher + Clone,
{
    type Output = Option<dashmap::mapref::one::Ref<'a, K, V>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.map.try_get(self.key) {
            dashmap::try_result::TryResult::Present(value) => Poll::Ready(Some(value)),
            dashmap::try_result::TryResult::Absent => Poll::Ready(None),
            dashmap::try_result::TryResult::Locked => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub struct GetMutFuture<'a, K, V, S> {
    key: &'a K,
    map: &'a dashmap::DashMap<K, V, S>,
}

impl<'a, K, V, S> Future for GetMutFuture<'a, K, V, S>
where
    K: std::hash::Hash + Eq + Clone,
    V: 'static,
    S: BuildHasher + Clone,
{
    type Output = Option<dashmap::mapref::one::RefMut<'a, K, V>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.map.try_get_mut(self.key) {
            dashmap::try_result::TryResult::Present(value) => Poll::Ready(Some(value)),
            dashmap::try_result::TryResult::Absent => Poll::Ready(None),
            dashmap::try_result::TryResult::Locked => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub struct InsertFuture<'a, K, V, S> {
    key: K,
    value: Option<V>,
    map: &'a dashmap::DashMap<K, V, S>,
}

impl<'a, K, V, S> Future for InsertFuture<'a, K, V, S>
where
    Self: Unpin,
    K: std::hash::Hash + Eq + Clone,
    V: 'static,
    S: BuildHasher + Clone,
{
    type Output = Option<V>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.map.try_entry(self.key.clone()) {
            Some(dashmap::Entry::Vacant(entry)) => {
                entry.insert_entry(
                    self.get_mut()
                        .value
                        .take()
                        .expect("future polled after completion"),
                );
                Poll::Ready(None)
            }
            Some(dashmap::Entry::Occupied(entry)) => {
                let (_, old) = entry.replace_entry(
                    self.get_mut()
                        .value
                        .take()
                        .expect("future polled after completion"),
                );
                Poll::Ready(Some(old))
            }
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

struct RemoveFuture<'a, K, V, S> {
    key: K,
    map: &'a dashmap::DashMap<K, V, S>,
}

impl<'a, K, V, S> Future for RemoveFuture<'a, K, V, S>
where
    K: std::hash::Hash + Eq + Clone,
    V: 'static,
    S: BuildHasher + Clone,
{
    type Output = Option<V>;

    #[inline(always)]
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.map.try_entry(self.key.clone()) {
            Some(entry) => match entry {
                dashmap::Entry::Occupied(old) => Poll::Ready(Some(old.remove())),
                dashmap::Entry::Vacant(_) => Poll::Ready(None),
            },
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

impl<'a, K, V, S> DashMapAsync<'a, K, V> for dashmap::DashMap<K, V, S>
where
    K: std::hash::Hash + Eq + Clone + Unpin + 'a,
    V: Unpin + 'static,
    S: BuildHasher + Clone,
{
    #[inline(always)]
    fn entry_async(&'a self, key: K) -> impl Future<Output = dashmap::Entry<'a, K, V>> {
        EntryFuture { key, map: self }
    }

    #[inline(always)]
    fn get_async<'b: 'a>(
        &'a self,
        key: &'b K,
    ) -> impl Future<Output = Option<dashmap::mapref::one::Ref<'_, K, V>>> {
        GetFuture { key, map: self }
    }

    #[inline(always)]
    fn get_mut_async<'b: 'a>(
        &'a self,
        key: &'b K,
    ) -> impl Future<Output = Option<dashmap::mapref::one::RefMut<'a, K, V>>> {
        GetMutFuture { key, map: self }
    }

    #[inline(always)]
    fn insert_async(&'a self, key: K, value: V) -> impl Future<Output = Option<V>> {
        InsertFuture {
            key,
            value: Some(value),
            map: self,
        }
    }

    #[inline(always)]
    fn remove_async(&'a self, key: K) -> impl Future<Output = Option<V>> {
        RemoveFuture { key, map: self }
    }
}

pub struct AsyncDashMap<K, V, S> {
    inner: dashmap::DashMap<K, V, S>,
}

impl<K, V> Default for AsyncDashMap<K, V, RandomState>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AsyncDashMap<K, V, RandomState>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: dashmap::DashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: dashmap::DashMap::with_capacity(capacity),
        }
    }
}

impl<'a, K, V, S> AsyncDashMap<K, V, S>
where
    K: std::hash::Hash + Eq + Clone + Unpin + 'static,
    V: Unpin + 'static,
    S: BuildHasher + Clone,
{
    #[allow(unused)]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            inner: dashmap::DashMap::with_hasher(hash_builder),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            inner: dashmap::DashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub async fn entry(&self, key: K) -> dashmap::mapref::entry::Entry<'_, K, V> {
        DashMapAsync::entry_async(&self.inner, key).await
    }

    #[inline(always)]
    pub async fn get<'b: 'a>(&'a self, key: &'b K) -> Option<dashmap::mapref::one::Ref<'a, K, V>> {
        DashMapAsync::get_async(&self.inner, key).await
    }

    #[inline(always)]
    pub async fn get_mut<'b: 'a>(
        &'a self,
        key: &'b K,
    ) -> Option<dashmap::mapref::one::RefMut<'_, K, V>> {
        DashMapAsync::get_mut_async(&self.inner, key).await
    }

    #[inline(always)]
    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        DashMapAsync::insert_async(&self.inner, key, value).await
    }

    #[inline(always)]
    pub async fn remove(&self, key: K) -> Option<V> {
        DashMapAsync::remove_async(&self.inner, key).await
    }
}

#[derive(Clone)]
pub struct AsyncDashMapTable<K, H>(Arc<AsyncDashMap<K, Value, H>>);

impl<K, H> Collection for AsyncDashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Unpin + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(AsyncDashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for AsyncDashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug + Unpin,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    async fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).await.is_some()
    }

    async fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).await.is_none()
    }

    async fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(*key).await.is_some()
    }

    async fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get_mut(key).await.map(|mut v| *v += 1).is_some()
    }
}
