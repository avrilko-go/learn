use crate::{KvError, KvPair, Storage, StorageIter, Value};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;

#[derive(Default, Debug)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, table: String) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(&table) {
            Some(table) => table,
            None => {
                let t = self.tables.entry(table).or_default();
                t.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: String, key: String) -> Result<Option<Value>, KvError> {
        let t = self.get_or_create_table(table);
        Ok(t.get(&key).map(|v| v.value().clone()))
    }

    fn set(&self, table: String, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let t = self.get_or_create_table(table);
        Ok(t.insert(key, value))
    }

    fn del(&self, table: String, key: String) -> Result<Option<Value>, KvError> {
        let t = self.get_or_create_table(table);
        Ok(t.remove(&key).map(|(_, v)| v))
    }

    fn contains(&self, table: String, key: String) -> Result<bool, KvError> {
        let t = self.get_or_create_table(table);
        Ok(t.contains_key(&key))
    }

    fn get_all(&self, table: String) -> Result<Vec<KvPair>, KvError> {
        let t = self.get_or_create_table(table);
        Ok(t.iter()
            .map(|v| KvPair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(&self, table: String) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError> {
        let t = self.get_or_create_table(table).clone();
        Ok(Box::new(StorageIter::new(t.into_iter())))
    }
}

impl From<(String, Value)> for KvPair {
    fn from(s: (String, Value)) -> Self {
        KvPair::new(s.0, s.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_or_create_should_work() {
        let mem_table = MemTable::new();
        assert_eq!(mem_table.tables.contains_key("t1"), false);
        mem_table.get_or_create_table("t1".to_string());
        assert!(mem_table.tables.contains_key("t1"));
    }
}
