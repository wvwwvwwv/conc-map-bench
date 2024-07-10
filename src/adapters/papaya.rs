use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

#[derive(Clone)]
pub struct PapayaMap<K: Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<papaya::HashMap<K, Value, H>>,
);

impl<K, H> Collection for PapayaMap<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(papaya::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for PapayaMap<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.pin().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.pin().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.pin().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.pin().update(*key, |v| *v + 1).is_some()
    }
}
