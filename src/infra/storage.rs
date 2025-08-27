use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use crate::{Result, ErrorCode};

pub trait Storage<K, V> {
    fn put(&self, key: K, value: V) -> Result<()>;
    fn get(&self, key: &K) -> Result<Option<V>> where V: Clone;
}

#[derive(Clone)]
pub struct InMemoryStorage<K, V> {
    inner: Arc<RwLock<HashMap<K, V>>>,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> Storage<K, V> for InMemoryStorage<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn put(&self, key: K, value: V) -> Result<()> {
        let mut map = self.inner.write().map_err(|_| ErrorCode::InvalidData)?;
        map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: &K) -> Result<Option<V>> {
        let map = self.inner.read().map_err(|_| ErrorCode::InvalidData)?;
        Ok(map.get(key).cloned())
    }
}

impl<K, V> InMemoryStorage<K, V> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            _phantom: PhantomData,
        }
    }
}

impl<K, V> Default for InMemoryStorage<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
