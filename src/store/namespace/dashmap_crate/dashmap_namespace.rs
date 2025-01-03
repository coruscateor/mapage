use dashmap::DashMap;

use std::{hash::Hash, collections::HashSet};

use crate::errors::{invalid_operation, key_not_found}; //types::{get_ok_value_str}, 

use std::mem::replace;

use anyhow::Result;

pub struct DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default + Clone
{

    map: DashMap<K, V>

}

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default + Clone
{

    pub fn new() -> Self
    {

        Self
        {

            map: DashMap::new()

        }

    }

    pub async fn insert(&self, key: K, value: V) -> Result<()>
    {
        
        let _res = self.map.insert(key, value);

        Ok(())

    }

    pub async fn update(&self, key: &K, value: V) -> Result<()>
    {

        let res = self.map.get_mut(&key);

        if let Some(mut val) = res
        {

            *val = value;

            return Ok(());

        }

        invalid_operation()

    }
    
    pub async fn replace(&self, key: &K, value: V) -> Result<V>
    {

        let res = self.map.get_mut(&key);
        
        if let Some(mut val) = res
        { 
        
            return Ok(replace(val.value_mut(), value));
        
        }

        key_not_found()

    }

    pub async fn try_replace(&self, key: &K, value: V) -> Option<V>
    {

        let res = self.map.get_mut(&key);
        
        if let Some(mut val) = res
        { 
        
            return Some(replace(val.value_mut(), value));
        
        }

        None

    }

    //calling functions etc

    //updater must return Result<R>

    pub async fn update_fn<R, FN>(&self, key: &K, updater: FN) -> Result<R>
        where FN: FnOnce(&mut V) -> Result<R>
    {

        let res = self.map.get_mut(&key);

        if let Some(mut val) = res
        {

            return updater(val.value_mut());

        }
        
        invalid_operation()

    }
    
    pub async fn update_kv_fn<R, FN>(&self, key: &K, updater: FN) -> Result<R>
        where FN: FnOnce(&K, &mut V) -> Result<R>
    {

        let res = self.map.get_mut(&key);

        if let Some(mut val) = res
        {

            return updater(key, val.value_mut());

        }
        
        invalid_operation()

    }

    pub async fn upsert(&self, key: K, value: V) -> Result<()>
    {
        
        let res = self.map.get_mut(&key);

        if let Some(mut ref_mut) = res
        {

            *ref_mut = value;

        }
        else
        {

            self.map.insert(key, value);

        }
        
        Ok(())

    }

    //

    pub async fn remove(&self, key: &K) -> Result<()>
    {

        let res = self.map.remove(key);

        if let Some(_) = res
        {

            return Ok(());

        }

        invalid_operation()

    }

    pub async fn retrieve(&self, key: &K) -> Result<V>
    {

        let res = self.try_retrieve(key);

        match res.await
        {

            Some(val) =>
            {

                Ok(val)

            }
            None =>
            {

                key_not_found()

            }

        }

    }

    pub async fn try_retrieve(&self, key: &K) -> Option<V>
    {

        let res = self.map.remove(key);

        if let Some(val) = res
        {

            return Some(val.1);

        }
        
        None

    }

    //read - calling functions

    //reader must return Result<R>

    pub async fn read_fn<R, FN>(&self, key: &K, reader: FN) -> Result<R>
        where FN: FnOnce(&V) -> Result<R>
    {

        let res = self.map.get(&key);

        if let Some(val) = res
        {

            return reader(val.value());

        }
        
        invalid_operation()

    }

    pub async fn read_kv_fn<R, FN>(&self, key: &K, reader: FN) -> Result<R>
        where FN: FnOnce(&K, &V) -> Result<R>
    {

        let res = self.map.get(&key);

        if let Some(val) = res
        {

            return reader(key, val.value());

        }
        
        invalid_operation()

    }

    //

    pub async fn contains(&self, key: &K) -> bool
    {

        self.map.contains_key(key)

    }

    pub async fn clear(&self)
    {

        self.map.clear();

    }

    pub async fn len_then_clear(&self) -> usize
    {

        let len = self.map.len();

        self.map.clear();

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

    //
    
    pub async fn read(&self, key: &K) -> Result<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Ok(val.value().clone());

        }
        
        invalid_operation()


    }

    pub async fn try_read(&self, key: &K) -> Option<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Some(val.value().clone());

        }

        None

    }
    
    pub async fn all_keys(&self) -> HashSet<K>
    {

        let mut keys = HashSet::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates

            let iter = self.map.iter();

            for item in iter
            {

                keys_ref.insert(item.key().clone());

            }

        }

        keys

    }

}

//Deprecated

//Copy - retriving values only

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default + Copy + Clone
{

    pub async fn upsert_copy(&self, key: K, value: V) -> Result<()>
    {

        self.upsert(key, value).await

    }

    pub async fn read_copy(&self, key: &K) -> Result<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Ok(*val.value());

        }
        
        invalid_operation()

    }

    pub async fn try_read_copy(&self, key: &K) -> Option<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Some(*val.value());

        }

        None

    }

}

//Clone - retriving values only

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default + Clone
{

    pub async fn upsert_clone(&self, key: K, value: V) -> Result<()>
    {

        self.upsert(key, value).await

    }
    
    pub async fn read_clone(&self, key: &K) -> Result<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Ok(val.value().clone());

        }
        
        invalid_operation()


    }

    pub async fn try_read_clone(&self, key: &K) -> Option<V>
    {

        let res = self.map.get(key);

        if let Some(val) = res
        {

            return Some(val.value().clone());

        }

        None

    }

}

//Copy keys

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Copy + Clone,
          V: 'static + Sync + Default + Clone //+ Copy
{

    pub async fn get_all_keys_copy(&self) -> Vec<K>
    {

        let mut keys = Vec::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates
    
            let iter = self.map.iter();

            for item in iter
            {

                if !keys_ref.contains(item.key())
                {
    
                    keys_ref.push(*item.key());
    
                }

            }

        }

        keys

    }

}

//Clone - retriving values only

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Clone,
          V: 'static + Sync + Default + Clone //+ Clone
{

    pub async fn get_all_keys_clone(&self) -> HashSet<K>
    {

        let mut keys = HashSet::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates

            let iter = self.map.iter();

            for item in iter
            {

                keys_ref.insert(item.key().clone());

            }

        }

        keys

    }

}
