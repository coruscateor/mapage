use scc::HashMap;

use std::{hash::Hash, collections::HashSet};

use crate::{errors::invalid_operation};

use crate::types::{get_ok_value_str};

use std::mem::replace;

//Non-async methods are appended with "_non_async" - features will probably be used actually

pub struct HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default
{

    map: HashMap<K, V>

}

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default
{

    pub fn new() -> Self
    {

        Self
        {

            map: HashMap::new()

        }

    }

    pub async fn insert(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {
        
        let res = self.map.insert_async(key, value);

        if let Err(_) = res.await
        {

            return invalid_operation();

        }

        Ok(get_ok_value_str())

    }

    pub async fn update(&self, key: &K, value: V) -> async_graphql::Result<&'static str>
    {

        let res = self.map.update_async(key, |_, v| { *v = value; });

        if let None = res.await
        {

            return invalid_operation();

        }

        Ok(get_ok_value_str())

    }

    pub async fn try_replace(&self, key: &K, value: V) -> Option<V>
    {

        let res = self.map.update_async(key, |_, v| { 
        
            replace(v, value)
        
        });

        res.await

    }

    //calling functions etc

    //updater must return async_graphql::Result<R>

    pub async fn update_fn<R, FN>(&self, key: &K, mut updater: FN) -> async_graphql::Result<R>
        where FN: FnMut(&mut V) -> async_graphql::Result<R>
    {

        let res = self.map.update_async(&key, |_, v| { updater(v) });

        if let Some(val) = res.await
        {

            return val;

        }
        
        invalid_operation()

    }

    pub async fn update_kv_fn<R, FN>(&self, key: &K, mut updater: FN) -> async_graphql::Result<R>
        where FN: FnMut(&K, &mut V) -> async_graphql::Result<R>
    {

        let res = self.map.update_async(&key, |k, v| { updater(k, v) });

        if let Some(val) = res.await
        {

            return val;
        }
        
        invalid_operation()

    }

    pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str>
    {

        let res = self.map.remove_async(key);

        if let None = res.await
        {

            return invalid_operation();

        }

        Ok(get_ok_value_str())

    }

    pub async fn try_retrieve(&self, key: &K) -> Option<V>
    {

        let res = self.map.remove_async(key);

        if let Some(val) = res.await
        {

            return Some(val.1);

        }
        
        None

    }

    //read - calling functions

    //reader must return async_graphql::Result<R>

    pub async fn read_fn<R, FN>(&self, key: &K, reader:FN) -> async_graphql::Result<R>
        where FN: Fn(&V) -> async_graphql::Result<R>
    {

        let res = self.map.read_async(&key, |_, v| { reader(v) });

        if let Some(val) = res.await
        {

            return val;

        }
        
        invalid_operation()

    }

    pub async fn read_kv_fn<R, FN>(&self, key: &K, reader: FN) -> async_graphql::Result<R>
        where FN: Fn(&K, &V) -> async_graphql::Result<R>
    {

        let res = self.map.read_async(&key, |k, v| { reader(k, v) });

        if let Some(val) = res.await
        {

            return val;

        }
        
        invalid_operation()

    }

    pub async fn contains(&self, key: &K) -> bool
    {

        self.map.contains_async(key).await

    }

    pub async fn clear(&self) -> &'static str
    {

        self.map.clear_async().await;

        get_ok_value_str()

    }

    pub async fn clear_and_get_len(&self) -> usize
    {

        let len = self.map.len();

        self.map.clear_async().await;

        len

    }

    pub async fn len(&self) -> usize
    {

        self.map.len()

    }

    pub async fn is_empty(&self) -> bool
    {

        self.map.is_empty()

    }

    pub async fn capacity(&self) -> usize
    {

        self.map.capacity()

    }

}

//Copy - retriving values only

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default + Copy
{

    pub async fn upsert_copy(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {

        let value_ref = &value; 

        self.map.entry(key).and_modify(|v| { *v = *value_ref; }).or_insert(value);

        Ok(get_ok_value_str())

    }

    pub async fn read_copy(&self, key: &K) -> async_graphql::Result<V>
    {

        let res = self.map.read_async(key, |_, v| *v);

        if let Some(val) = res.await
        {

            return Ok(val);

        }
        
        invalid_operation()

    }

    pub async fn try_read_copy(&self, key: &K) -> Option<V>
    {

        self.map.read_async(key, |_, v| *v).await

    }

}

//Clone - retriving values only

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default + Clone
{

    pub async fn upsert_clone(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {

        let value_ref = &value; 

        self.map.entry(key).and_modify(|v| { *v = value_ref.clone(); }).or_insert(value);

        Ok(get_ok_value_str())

    }

    pub async fn read_clone(&self, key: &K) -> async_graphql::Result<V>
    {

        let res = self.map.read_async(key, |_, v| v.clone());

        if let Some(val) = res.await
        {

            return Ok(val);

        }
        
        invalid_operation()

    }

    pub async fn try_read_clone(&self, key: &K) -> Option<V>
    {

        self.map.read_async(key, |_, v| v.clone()).await

    }

}

//Copy keys

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Copy,
          V: 'static + Sync + Default
{

    pub async fn get_all_keys_copy(&self) -> HashSet<K>
    {

        let mut keys = HashSet::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates
    
            self.map.scan_async(|k, _| { 
                
                keys_ref.insert(*k);

            }).await;

        }

        keys

    }


}

//Clone - retriving values only

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default
{

    pub async fn get_all_keys_clone(&self) -> HashSet<K>
    {

        let mut keys = HashSet::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates
    
            self.map.scan_async(|k, _| { 
                
                keys_ref.insert(k.clone());

            }).await;

        }

        keys

    }

}


