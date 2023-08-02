use scc::HashMap;

use std::{hash::Hash, collections::HashSet};

use crate::{errors::invalid_operation}; //types::UnitValue,

use crate::types::{get_ok_value_str};

use std::mem::replace;

//use paste::paste;

//Non-async methods are appended with "_non_async" - features will probably be used actually

/*
#[macro_export]
macro_rules! impl_scc_hashmap_generic_update_fn_param
{

    ($parameter_name:ident, $type_param:ty) => //ident
    {

        paste! {

            pub async fn update_fn_1_param<$type_param, R>(&self, key: K, updater: fn(&mut V, $type_param) -> async_graphql::Result<R>, $parameter_name: $type_param) -> async_graphql::Result<R>
            {
        
                let res = self.map.update_async(&key, |_, v| { updater(v, $parameter_name) });
                
                if let Some(val) = res.await
                {
        
                    return val;

                }
                
                invalid_operation()
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_generic_update_fn_params
{

    ($param_count:stmt, $($parameter_name:ident: $type_param:ty),*) => //ident
    {

        paste! {

            pub async fn [<update_fn_ $param_count _params>]<$($type_param),*, R>(&self, key: K, updater: fn(&mut V, $($type_param),*) -> async_graphql::Result<R>, $($parameter_name: $type_param),*) -> async_graphql::Result<R>
            {
        
                let res = self.map.update_async(&key, |_, v| { updater(v, $($parameter_name),*) });
                
                if let Some(val) = res.await
                {
        
                    return val;

                }
                
                invalid_operation()
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_generic_read_fn_params
{

    ($param_count:stmt, $($parameter_name:ident: $type_param:ty),*) => //$key_type:ty, //ident
    {

        paste! {

            pub async fn [<read_fn_ $param_count _params>]<$($type_param),*, R>(&self, key: K, reader: fn(&V, $($type_param),*) -> async_graphql::Result<R>, $($parameter_name: $type_param),*) -> async_graphql::Result<R>
            {
        
                let res = self.map.read_async(&key, |_, v| { reader(v, $($parameter_name),*) });
                
                if let Some(val) = res.await
                {
        
                    return val;
        
                }
                
                invalid_operation()
        
            }

        }

    }

}
*/

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

    pub async fn insert(&self, key: K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {
        
        let res = self.map.insert_async(key, value);

        if let Err(_) = res.await
        {

            return invalid_operation();

        }

        //Ok(UnitValue::new())

        Ok(get_ok_value_str())

    }

    pub async fn update(&self, key: &K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let res = self.map.update_async(key, |_, v| { *v = value; /*()*/ });

        if let None = res.await
        {

            return invalid_operation();

        }
        
        //Ok(UnitValue::new())

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

    pub async fn update_fn<R, FN>(&self, key: &K, mut updater: FN) -> async_graphql::Result<R> //F, //&K,   //F) -> async_graphql::Result<R> //<UnitValue> //|&K, &mut V| -> async_graphql::Result<R>) -> async_graphql::Result<R> //
        where FN: FnMut(&mut V) -> async_graphql::Result<R>
        //where F: FnOnce(&K, &mut V) -> async_graphql::Result<R>,
        //where F: fn(&K, &mut V) -> async_graphql::Result<R>,
    {

        //let ud = |k, v| { updater(k, v) };

        let res = self.map.update_async(&key, |_, v| { updater(v) }); //|k, v| { updater(k, v) });

        //if let None = res.await
        if let Some(val) = res.await
        {

            return val; //invalid_operation();

        }
        
        invalid_operation()

        //Ok(UnitValue::new())

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

    //param

    //($label:ident, $($parameter_name:ident: $type_param:ty),*)

    //crate::impl_scc_hashmap_generic_update_fn_params!(1, param1: P1);

    /*
    crate::impl_scc_hashmap_generic_update_fn_param!(param, P);

    crate::impl_scc_hashmap_generic_update_fn_params!(2, param1: P1, param2: P2);

    crate::impl_scc_hashmap_generic_update_fn_params!(3, param1: P1, param2: P2, param3: P3);

    crate::impl_scc_hashmap_generic_update_fn_params!(4, param1: P1, param2: P2, param3: P3, param4: P4);
    */

    /*
    pub async fn upsert(&self, key: K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let mut value_ref = &value; 
        
        self.map.upsert_async(key, || *value_ref, |_, v| { *v = *value_ref; }).await;
        
        //Ok(UnitValue::new())

        Ok(get_ok_value_str())

    }
    */

    /*
    pub async fn update_fn_param<P, R>(&self, key: K, updater: fn(&mut V, P) -> async_graphql::Result<R>, param: P) -> async_graphql::Result<R> //F, //&K, //updater: F, param: P) -> async_graphql::Result<R> //<UnitValue>
        //where F: FnOnce(&K, &mut V, P) -> async_graphql::Result<R>,
    {

        let res = self.map.update_async(&key, |_, v| { updater(v, param) }); //k,

        //if let None = res.await
        if let Some(val) = res.await
        {

            return val; //invalid_operation();

        }
        
        invalid_operation()

        //Ok(UnitValue::new())

    }
    */

    //convert the result to valid async-graphql type

    //IR - inital result

    /*
    pub async fn update_fns_convert<F, P, IR, C, R>(&self, key: K, updater: F, converter: C) -> async_graphql::Result<R>
        where F: FnOnce(&K, &mut V) -> IR,
              C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.update_async(&key, |k, v| { updater(k, v) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }

    //param

    pub async fn update_fns_convert_param<F, P, IR, C, R>(&self, key: K, updater: F, converter: C, param: P) -> async_graphql::Result<R>
        where F: FnOnce(&K, &mut V, P) -> IR,
            C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.update_async(&key, |k, v| { updater(k, v, param) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }
    */

    //

    /*
    pub async fn upsert(&self, key: K, value: V) -> async_graphql::Result<UnitValue>
    {

        let value_ref = &value; 
        
        self.map.upsert_async(key, move || *value_ref, move |_, v| { *v = *value_ref; /*()*/ }).await; //V::default()
        
        Ok(UnitValue::new())

    }
    */

    //

    /*
    pub async fn upsert_fn<F, R>(&self, key: K, updater: F) -> async_graphql::Result<UnitValue> //<R> //<UnitValue>
        where F: FnOnce(&K, &mut V)
    {

        let res = self.map.upsert_async(key, || V::default(), |k, v| { updater(k, v) });

        Ok(UnitValue::new())

        //if let None = res.await

        /*
        if let Some(thing) = res.await
        {

            return Ok(thing); //invalid_operation();

        }
        
        invalid_operation()
        */

        //Ok(UnitValue::new())

    }

    //param

    pub async fn upsert_fn_param<F, P, R>(&self, key: K, updater: F, param: P) -> async_graphql::Result<R> //<UnitValue>
        where F: FnOnce(&K, &mut V, P) -> R,
    {

        self.map.upsert_async(&key, |k, v| { updater(k, v, param) }).await;

        Ok(UnitValue::new())

    }

    //convert the result to valid async-graphql type

    //IR - inital result

    pub async fn upsert_fns_convert<F, P, IR, C, R>(&self, key: K, updater: F, converter: C) -> async_graphql::Result<R>
        where F: FnOnce(&K, &mut V) -> IR,
              C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.upsert_async(&key, |k, v| { updater(k, v) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }

    //param

    pub async fn upsert_fns_convert_param<F, P, IR, C, R>(&self, key: K, updater: F, converter: C, param: P) -> async_graphql::Result<R>
        where F: FnOnce(&K, &mut V, P) -> IR,
            C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.upsert_async(&key, |k, v| { updater(k, v, param) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }
    */

    //

    pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let res = self.map.remove_async(key);

        if let None = res.await
        {

            return invalid_operation();

        }
        
        //Ok(UnitValue::new())

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

    pub async fn read_fn<R, FN>(&self, key: &K, reader:FN) -> async_graphql::Result<R> //&K, //F, //<UnitValue>
        where FN: Fn(&V) -> async_graphql::Result<R>
        //where F: FnMut(&K, &V) -> async_graphql::Result<R>,
    {

        let res = self.map.read_async(&key, |_, v| { reader(v) }); //|k, v| { reader(k, v) });

        //if let None = res.await
        if let Some(val) = res.await
        {

            return val; //invalid_operation();

        }
        
        invalid_operation()

        //Ok(UnitValue::new())

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

    //params

    /*
    pub async fn read_fn_param<P, R>(&self, key: K, reader: fn(&V, &P) -> async_graphql::Result<R>, param: &P) -> async_graphql::Result<R> //&K, //F, //<UnitValue>
        //where F: Fn(&K, &V, &P) -> async_graphql::Result<R>,
        //where F: Fn(&K, &V, &P) -> R,
    {

        let res = self.map.read_async(&key, |_, v| { reader(v, param) });

        //if let None = res.await
        if let Some(val) = res.await
        {

            return val; //invalid_operation();

        }
        
        invalid_operation()

        //Ok(UnitValue::new())

    }
    */

    //convert the result to valid async-graphql type

    //IR - inital result

    /*
    pub async fn read_fns_convert<F, P, IR, C, R>(&self, key: K, reader: F, converter: C) -> async_graphql::Result<R>
        where F: Fn(&K, &V) -> IR,
            C: Fn(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.read_async(&key, |k, v| { reader(k, v) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }

    //param

    pub async fn read_fns_convert_param<F, P, IR, C, R>(&self, key: K, reader: F, converter: C, param: &P) -> async_graphql::Result<R>
        where F: Fn(&K, &V, &P) -> IR,
            C: Fn(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.read_async(&key, |k, v| { reader(k, v, param) });

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }
    */

    //convert the result to valid async-graphql type

    //IR - inital result

    /*
    pub async fn read_fns_convert<F, IR, C, R>(&self, key: K, reader: F, converter: C) -> async_graphql::Result<R>
        where F: FnMut(&K, &V) -> IR,
              C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.read_async(&key, reader);

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }
    */

    //

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

        self.map.clear_async().await

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

    pub async fn upsert_copy(&self, key: K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let value_ref = &value; 
        
        self.map.upsert_async(key, || *value_ref, |_, v| { *v = *value_ref; }).await;
        
        //Ok(UnitValue::new())

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

    /*
    pub async fn try_swap_copy(&self, key: K, value: V) -> Option<V>
    {

        let res = self.map.update_async(&key, |_, v| { 
        
            let return_value = *v;
            
            *v = value;

            return_value
        
        });

        res.await

    }
    */

    //calling functions

    /*
    pub async fn read_fn_copy<F, R>(&self, key: K, reader: F) -> async_graphql::Result<R> //<UnitValue>
        where F: FnMut(&K, &V) -> R,
    {

        let res = self.map.read_async(&key, reader); //|k, v| { reader(k, v) });

        //if let None = res.await
        if let Some(thing) = res.await
        {

            return Ok(thing); //invalid_operation();

        }
        
        invalid_operation()

        //Ok(UnitValue::new())

    }

    //convert the result to valid async-graphql type

    //IR - inital result

    pub async fn read_fns_convert_copy<F, IR, C, R>(&self, key: K, reader: F, converter: C) -> async_graphql::Result<R>
        where F: FnMut(&K, &V) -> IR,
            C: FnOnce(IR) -> async_graphql::Result<R> //&mut 
    {

        let res = self.map.read_async(&key, reader);

        if let Some(thing) = res.await
        {

            return converter(thing);

        }
        
        invalid_operation()

    }
    */
}

//Clone - retriving values only

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync,
          V: 'static + Sync + Default + Clone
{

    pub async fn upsert_clone(&self, key: K, value: V) -> async_graphql::Result<&'static str> //<UnitValue>
    {

        let value_ref = &value; 
        
        self.map.upsert_async(key, || value_ref.clone(), |_, v| { *v = value.clone(); }).await;
        
        //self.map.upsert_async(key, || *value_ref, |_, v| { *v = *value_ref;}).await;

        //Ok(UnitValue::new())

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

    /*
    pub async fn try_swap_clone(&self, key: K, value: V) -> Option<V>
    {

        let res = self.map.update_async(&key, |_, v| { 
        
            let return_value = (*v).clone();
            
            *v = value;

            return_value
        
        });

        res.await

    }
    */

}

//Copy keys

impl<K, V> HashMapNamespace<K, V>
    where K: 'static + Eq + Hash + Sync + Copy,
          V: 'static + Sync + Default //+ Copy
{

    /*
    pub async fn get_all_keys_copy(&self) -> Vec<K>
    {

        let mut keys = Vec::with_capacity(self.map.len());

        {
            let keys_ref = &mut keys;

            //May need to check for duplicates
    
            self.map.scan_async(|k, _| { 
                
                if !keys_ref.contains(k)
                {

                    keys_ref.push(*k);

                }

            }).await;

        }

        keys

    }
    */

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
          V: 'static + Sync + Default //+ Clone
{

    /*
    pub async fn get_all_keys_clone(&self) -> Vec<K>
    {

        let mut keys = Vec::with_capacity(self.map.len());

        {

            let keys_ref = &mut keys;

            //May need to check for duplicates
    
            self.map.scan_async(|k, _| { 
                
                if !keys_ref.contains(k)
                {
                
                    keys_ref.push(k.clone());
            
                }
            
            }).await;

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
    
            self.map.scan_async(|k, _| { 
                
                keys_ref.insert(k.clone());

            }).await;

        }

        keys

    }

}
