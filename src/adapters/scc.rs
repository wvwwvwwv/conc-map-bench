use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

#[derive(Clone)]
pub struct SccTable<K: Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<scc::HashMap<K, Value, H>>,
);

impl<K, H> Collection for SccTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(scc::HashMap::new(capacity, H::default())))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
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
