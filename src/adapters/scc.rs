use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

#[derive(Clone)]
pub struct SccMap<K: Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<scc::HashMap<K, Value, H>>,
);

impl<K, H> Collection for SccMap<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        for _ in 0..16384 {
            drop(scc::ebr::Barrier::new());
        }
        Self(Arc::new(scc::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccMap<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read(key, |_, _| ()).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.update(key, |_, v| *v += 1).is_some()
    }
}

#[derive(Clone)]
pub struct SccIndex<K: Clone + Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<scc::HashIndex<K, Value, H>>,
);

impl<K, H> Collection for SccIndex<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        for _ in 0..16384 {
            drop(scc::ebr::Barrier::new());
        }
        Self(Arc::new(scc::HashIndex::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccIndex<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read(key, |_, _| ()).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        unsafe { self.0.update(key, |_, v| *v += 1).is_some() }
    }
}
