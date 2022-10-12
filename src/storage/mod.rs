pub mod memory;

use crate::{KvError, KvPair, Value};

pub trait Storage: Send + Sync + 'static {
    fn get(&self, table: String, key: String) -> Result<Option<Value>, KvError>;

    fn set(&self, table: String, key: String, value: Value) -> Result<Option<Value>, KvError>;

    fn del(&self, table: String, key: String) -> Result<Option<Value>, KvError>;

    fn contains(&self, table: String, key: String) -> Result<bool, KvError>;

    fn get_all(&self, table: String) -> Result<Vec<KvPair>, KvError>;

    fn get_iter(&self, table: String) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError>;
}

pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<KvPair>,
{
    type Item = KvPair;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::memory::MemTable;

    #[test]
    fn mem_table_base_interface_should_work() {
        let mem_table = MemTable::new();
        test_base_interface(mem_table);
    }

    #[test]
    fn mem_table_get_all_should_work() {
        let mem_table = MemTable::new();
        test_get_all(mem_table);
    }

    #[test]
    fn mem_table_get_iter_should_work() {
        let mem_table = MemTable::new();
        test_get_iter(mem_table);
    }

    fn test_base_interface(store: impl Storage) {
        assert!(store
            .set("t1".to_string(), "key1".to_string(), "a".into())
            .unwrap()
            .is_none());
        assert_eq!(
            store
                .set("t1".to_string(), "key1".to_string(), "b".into())
                .unwrap(),
            Some("a".into())
        );

        assert_eq!(
            store.get("t1".to_string(), "key1".to_string()).unwrap(),
            Some("b".into())
        );
        assert_eq!(
            store.get("t1".to_string(), "key2".to_string()).unwrap(),
            None
        );
        assert_eq!(
            store.get("t2".to_string(), "key1".to_string()).unwrap(),
            None
        );

        assert_eq!(
            store
                .contains("t1".to_string(), "key1".to_string())
                .unwrap(),
            true
        );
        assert_eq!(
            store
                .contains("t2".to_string(), "key1".to_string())
                .unwrap(),
            false
        );
        assert_eq!(
            store
                .contains("t1".to_string(), "key2".to_string())
                .unwrap(),
            false
        );

        assert_eq!(
            store.del("t1".to_string(), "key1".to_string()).unwrap(),
            Some("b".into())
        );
        assert_eq!(
            store.del("t1".to_string(), "key2".to_string()).unwrap(),
            None
        );
    }

    fn test_get_all(store: impl Storage) {
        store
            .set("t1".to_string(), "key1".to_string(), "a".into())
            .unwrap();
        store
            .set("t1".to_string(), "key2".to_string(), "b".into())
            .unwrap();

        let mut data = store.get_all("t1".to_string()).unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                KvPair::new("key1", "a".into()),
                KvPair::new("key2", "b".into()),
            ]
        );
    }

    fn test_get_iter(store: impl Storage) {
        store
            .set("t1".to_string(), "key1".to_string(), "a".into())
            .unwrap();
        store
            .set("t1".to_string(), "key2".to_string(), "b".into())
            .unwrap();

        let mut data: Vec<KvPair> = store.get_iter("t1".to_string()).unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                KvPair::new("key1", "a".into()),
                KvPair::new("key2", "b".into()),
            ]
        );
    }
}
