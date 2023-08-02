use dashmap::DashMap;

use std::{hash::Hash, collections::HashSet};

use crate::{types::{get_ok_value_str}, errors::invalid_operation}; //UnitValue, 

use std::mem::replace;

//use paste::paste;

/*
#[macro_export]
macro_rules! impl_dashmap_generic_update_fn_param
{

    ($parameter_name:ident, $type_param:ty) =>
    {

        paste! {

            pub async fn update_fn_1_param<$type_param, R>(&self, key: K, updater: fn(&mut V, $type_param) -> async_graphql::Result<R>, $parameter_name: $type_param) -> async_graphql::Result<R>
            {

                let res = self.map.get_mut(&key);

                if let Some(mut val) = res
                { 
                
                    return updater(val.value_mut(), $parameter_name);
                
                }
                
                invalid_operation()
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_dashmap_generic_update_fn_params
{

    ($param_count:stmt, $($parameter_name:ident: $type_param:ty),*) => //ident
    {

        paste! {

            pub async fn [<update_fn_ $param_count _params>]<$($type_param),*, R>(&self, key: K, updater: fn(&mut V, $($type_param),*) -> async_graphql::Result<R>, $($parameter_name: $type_param),*) -> async_graphql::Result<R>
            {
                
                let res = self.map.get_mut(&key);

                if let Some(mut val) = res
                { 
                
                    return updater(val.value_mut(), $($parameter_name),*);
                
                }
                
                invalid_operation()
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_dashmap_generic_read_fn_params
{

    ($param_count:stmt, $($parameter_name:ident: $type_param:ty),*) => //$key_type:ty, //ident
    {

        paste! {

            pub async fn [<read_fn_ $param_count _params>]<$($type_param),*, R>(&self, key: K, reader: fn(&V, $($type_param),*) -> async_graphql::Result<R>, $($parameter_name: $type_param),*) -> async_graphql::Result<R>
            {
                
                let res = self.map.get(&key);

                if let Some(val) = res
                {
                
                    return reader(val.value(), $($parameter_name),*);
                
                }
                
                invalid_operation()
        
            }

        }

    }

}
*/

pub struct DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default
{

    map: DashMap<K, V>

}

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default
{

    pub fn new() -> Self
    {

        Self
        {

            map: DashMap::new()

        }

    }

    pub async fn insert(&self, key: K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {
        
        let res = self.map.insert(key, value);

        //Ok(UnitValue::new())

        Ok(get_ok_value_str())

    }

    pub async fn update(&self, key: &K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let res = self.map.get_mut(&key);

        if let Some(mut val) = res
        {

            *val = value;

            //return Ok(UnitValue::new());

            return Ok(get_ok_value_str());

        }

        invalid_operation()

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

    //updater must return async_graphql::Result<R>

    pub async fn update_fn<R, FN>(&self, key: &K, mut updater: FN) -> async_graphql::Result<R>
        where FN: FnMut(&mut V) -> async_graphql::Result<R>
    {

        let res = self.map.get_mut(&key); //.update_async(&key, |_, v| { updater(v) });

        if let Some(mut val) = res
        {

            return updater(val.value_mut());

        }
        
        invalid_operation()

    }

    pub async fn update_kv_fn<R, FN>(&self, key: &K, mut updater: FN) -> async_graphql::Result<R>
        where FN: FnMut(&K, &mut V) -> async_graphql::Result<R>
    {

        let res = self.map.get_mut(&key);

        if let Some(mut val) = res
        {

            return updater(key, val.value_mut());

        }
        
        invalid_operation()

    }

    //param

    /*
    crate::impl_dashmap_generic_update_fn_param!(param, P);

    crate::impl_dashmap_generic_update_fn_params!(2, param1: P1, param2: P2);

    crate::impl_dashmap_generic_update_fn_params!(3, param1: P1, param2: P2, param3: P3);

    crate::impl_dashmap_generic_update_fn_params!(4, param1: P1, param2: P2, param3: P3, param4: P4);
    */

    //

    pub async fn upsert(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {

        let value_ref = &value; 
        
        let res = self.map.get_mut(&key);  //upsert_async(key, || *value_ref, |_, v| { *v = *value_ref; }).await;

        if let Some(mut ref_mut) = res
        {

            *ref_mut = value;

        }
        else
        {

            self.map.insert(key, value);

        }
        
        Ok(get_ok_value_str())

    }

    //

    pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let res = self.map.remove(key);

        if let Some(_) = res
        {

            //return Ok(UnitValue::new());

            return Ok(get_ok_value_str());

        }

        invalid_operation()

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

    //reader must return async_graphql::Result<R>

    pub async fn read_fn<R, FN>(&self, key: &K, reader: FN) -> async_graphql::Result<R>
        where FN: Fn(&V) -> async_graphql::Result<R>
    {

        let res = self.map.get(&key); //, |_, v| { reader(v) });

        if let Some(val) = res
        {

            return reader(val.value());

        }
        
        invalid_operation()

    }

    pub async fn read_kv_fn<R, FN>(&self, key: &K, reader: FN) -> async_graphql::Result<R>
        where FN: Fn(&K, &V) -> async_graphql::Result<R>
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

    pub async fn clear(&self) -> &'static str
    {

        self.map.clear();

        get_ok_value_str()

    }

    pub async fn clear_and_get_len(&self) -> usize
    {

        let len = self.map.len();

        self.map.clear();

        len

    }


    /*
    pub async fn clear(&self)
    {

        self.map.clear()

    }
    */

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

impl<K, V> DashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default + Copy
{

    pub async fn upsert_copy(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {

        self.upsert(key, value).await

    }

    pub async fn read_copy(&self, key: &K) -> async_graphql::Result<V>
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
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default + Clone
{

    pub async fn upsert_clone(&self, key: K, value: V) -> async_graphql::Result<&'static str>
    {

        self.upsert(key, value).await

    }
    
    pub async fn read_clone(&self, key: &K) -> async_graphql::Result<V>
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
    where K: 'static + Eq + Hash + Sync + Copy,
          V: 'static + Sync + Default //+ Copy
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
          V: 'static + Sync + Default //+ Clone
{

    /*
    pub async fn get_all_keys_clone(&self) -> Vec<K>
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
    
                    keys_ref.push(item.key().clone());
    
                }

            }

        }

        keys

    }
    */

    pub async fn get_all_keys_clone(&self) -> HashSet<K>
    {

        let mut keys = HashSet::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates

            let iter = self.map.iter();

            for item in iter //self.map 
            {

                //keys_ref.insert(k.clone());

                keys_ref.insert(item.key().clone());

            }

        }

        keys

    }

}
