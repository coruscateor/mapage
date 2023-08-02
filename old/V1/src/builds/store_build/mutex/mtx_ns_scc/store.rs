use std::collections::hash_map::RandomState;

use async_graphql::futures_util::lock::{Mutex, MutexGuard};
use scc::HashMap;
use tokio::time::error::Elapsed;

use super::namespace::Namespace;

use act_rusty::*;

use crate::async_graphql_types::*;
use crate::async_graphql_value_containers::{AnyObject, NumericOrBool, I128Scalar, U128Scalar, AnyInputObject};
use crate::binary_operations::Bop;
use crate::consts::UnitValue;
use crate::errors::*;
use crate::identifier::Identifier;
use crate::namespace_getters::NamespaceGetters;
use crate::namespace_setters_mut::NamespaceSettersMut;
use crate::uniary_operations::Uop;
use crate::{stored_object::StoredObject, all_types::*};

use crate::storage_container::*;

use crate::actor_utils::*;

use std::collections::HashSet;

///Evey Namespace is contained in a Mutex
pub struct Store
{

    default_namespace: Mtx<Namespace>,
    namespaces: HashMap<String, Mtx<Namespace>>

}

impl Store
{

    pub fn new() -> Self
    {

        Self
        {

            default_namespace: Mtx::new(Namespace::new()),
            namespaces: HashMap::new() //(1000, RandomState::new())

        }

    }

    //Setup

    fn update_or_default_ns<T, F>(&self, identifier: &Identifier, f: F) -> async_graphql::Result<T>
        where F: Fn(&Identifier, MtxGuard<Namespace>) -> async_graphql::Result<T>
    {

        let res;

        if let Some(ns) = identifier.get_namespace_ref()
        {

            let opt_res = self.namespaces.update(ns, |_, locked_val| {

                let val = locked_val.lock().unwrap();

                f(identifier, val)

            });

            res = opt_res.unwrap_or_else(|| invalid_operation());

        }
        else
        {

            let val = self.default_namespace.lock().unwrap();

            res = f(identifier, val);

        }

        res

    }

    async fn async_update_or_default_ns<T, F>(&self, identifier: &Identifier, f: F) -> async_graphql::Result<T>
        where F: Fn(&Identifier, MtxGuard<Namespace>) -> async_graphql::Result<T>
    {

        let res;

        if let Some(ns) = identifier.get_namespace_ref()
        {

            let opt_res = self.namespaces.update_async(ns, |_, locked_val| {

                let val = locked_val.lock().unwrap();

                f(identifier, val)

            }).await;

            res = opt_res.unwrap_or_else(|| invalid_operation());

        }
        else
        {

            let val = self.default_namespace.lock().unwrap();

            res = f(identifier, val);

        }

        res

    }

    fn update_or_default_ns_param<T, F, P>(&self, identifier: &Identifier, p: P, f: F) -> async_graphql::Result<T>
    where F: Fn(&Identifier, MtxGuard<Namespace>, P) -> async_graphql::Result<T>
    {

        let res;

        if let Some(ns) = identifier.get_namespace_ref()
        {

            let opt_res = self.namespaces.update(ns, |_, locked_val| {

                let val = locked_val.lock().unwrap();

                f(identifier, val, p)

            });

            res = opt_res.unwrap_or_else(|| invalid_operation());

        }
        else
        {

            let val = self.default_namespace.lock().unwrap();

            res = f(identifier, val, p);

        }

        res

    }

    //

    fn update_or_default_get_ns<T, F>(&self, namespace: &Option<String>, f: F) -> async_graphql::Result<T>
        where F: Fn(&Option<String>, MtxGuard<Namespace>) -> async_graphql::Result<T>, 
    {

        let res;

        if let Some(ns) = namespace.as_ref()
        {

            let opt_res = self.namespaces.update(ns, |_, locked_val| {

                let val = locked_val.lock().unwrap();

                f(&namespace, val)

            });

            res = opt_res.unwrap_or_else(|| invalid_operation());

        }
        else
        {

            let val = self.default_namespace.lock().unwrap();

            res = f(&namespace, val);

        }

        res

    }
    
    fn update_or_default_get_ns_differentiated<T, F_FOUND, F_DEFAULT>(&self, namespace: &Option<String>, f_found_ns: F_FOUND, f_default_ns: F_DEFAULT) -> async_graphql::Result<T>
        where F_FOUND: Fn(&String, MtxGuard<Namespace>) -> async_graphql::Result<T>, 
              F_DEFAULT: Fn(MtxGuard<Namespace>) -> async_graphql::Result<T>
    {

        let res;

        if let Some(ns) = namespace.as_ref()
        {

            let opt_res = self.namespaces.update(ns, |_, locked_val| {

                let val = locked_val.lock().unwrap();

                f_found_ns(ns, val)

            });

            res = opt_res.unwrap_or_else(|| invalid_operation());

        }
        else
        {

            let val = self.default_namespace.lock().unwrap();

            res = f_default_ns(val);

        }

        res

    }

    //Queries

    //Administration

    pub fn contains_namespace(&self, name: String) -> bool //async_graphql::Result<bool>
    {

        self.namespaces.contains(&name)

    }

    pub fn get_all_namespace_names(&self) -> HashSet<String>
    {
        
        let mut set = HashSet::new();

        let mut set_ref = &mut set;

        self.namespaces.for_each(|name, _| {

            set_ref.insert(name.clone());

        });

        set

    }

    pub fn count_default_namespaced_values(&self) -> usize
    {

        let lk = self.default_namespace.lock().unwrap();

        lk.get_count()

    }

    pub fn count_namespaced_values(&self, name: String) -> Option<usize>
    {

        let res = self.namespaces.read(&name, |name, mtx| {

            let ns = mtx.lock().unwrap();

            ns.get_count()

        });

        res

    }

    pub fn get_default_namespace_value_names(&self) -> Vec<String>
    {

        let lk = self.default_namespace.lock().unwrap();

        lk.get_value_names()

    }

    pub fn get_namespace_names(&self, name: String) -> Option<Vec<String>>
    {

        self.namespaces.read(&name, |_, mtx|
        {

            let ns = mtx.lock().unwrap();
            
            ns.get_value_names()

        })

    }

    //count_all_namespaced_values

    /*
    pub fn count_all_values(&self) -> usize
    {

        let mut count: usize = 0;

        let count_ref = &mut count;

        self.namespaces.for_each(|_, namespace| {

            let lk = namespace.lock().unwrap();

            *count_ref += lk.get_count();

        });

        {

            let lk = self.default_namespace.lock().unwrap();

            count += lk.get_count();

        }

        count

    }
    */

    /*
    pub fn get_all_namespace_names_and_types(&self) -> HashMap<String, Type>
    {
        
        let mut map = HashMap::new(1000, RandomState::new());

        let mut map_ref = &mut map;

        self.namespaces.for_each(|name, _| {

            map_ref.insert(name.clone());

        });

        map

    }
    */


    pub fn namespaces_count(&self) -> usize
    {

        self.namespaces.len()

    }

    //NamespaceGetters

    pub async fn get_value(&self, identifier: Identifier) -> async_graphql::Result<Option<AnyObject>>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: std::sync::MutexGuard<Namespace>| {

            locked_ns.get_value(identifier)

        })

    }

    pub async fn get_bool(&self, identifier: Identifier) -> async_graphql::Result<bool>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_bool(identifier)

        })
        
    }

    pub async fn get_char(&self, identifier: Identifier) -> async_graphql::Result<char>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_char(identifier)

        })
        
    }

    pub async fn get_f32(&self, identifier: Identifier) -> async_graphql::Result<f32>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_f32(identifier)

        })
        
    }

    pub async fn get_f64(&self, identifier: Identifier) -> async_graphql::Result<f64>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_f64(identifier)

        })
        
    }

    pub async fn get_i8(&self, identifier: Identifier) -> async_graphql::Result<i8>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_i8(identifier)

        })
        
    }

    pub async fn get_i16(&self, identifier: Identifier) -> async_graphql::Result<i16>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_i16(identifier)

        })
        
    }

    pub async fn get_i32(&self, identifier: Identifier) -> async_graphql::Result<i32>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_i32(identifier)

        })
        
    }

    pub async fn get_i64(&self, identifier: Identifier) -> async_graphql::Result<i64>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_i64(identifier)

        })
        
    }

    pub async fn get_i128(&self, identifier: Identifier) -> async_graphql::Result<I128Scalar>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_i128(identifier)

        })
        
    }

    pub async fn get_isize(&self, identifier: Identifier) -> async_graphql::Result<isize>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_isize(identifier)

        })
        
    }

    //

    pub async fn get_u8(&self, identifier: Identifier) -> async_graphql::Result<u8>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_u8(identifier)

        })
        
    }

    pub async fn get_u16(&self, identifier: Identifier) -> async_graphql::Result<u16>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_u16(identifier)

        })
        
    }

    pub async fn get_u32(&self, identifier: Identifier) -> async_graphql::Result<u32>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_u32(identifier)

        })
        
    }

    pub async fn get_u64(&self, identifier: Identifier) -> async_graphql::Result<u64>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_u64(identifier)

        })
        
    }

    pub async fn get_u128(&self, identifier: Identifier) -> async_graphql::Result<U128Scalar>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_u128(identifier)

        })
        
    }

    pub async fn get_unit(&self, identifier: Identifier) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_unit(identifier)

        })
        
    }

    pub async fn get_usize(&self, identifier: Identifier) -> async_graphql::Result<usize>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_usize(identifier)

        })
        
    }

    //

    pub async fn get_string(&self, identifier: Identifier) -> async_graphql::Result<String>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_string(identifier)

        })
        
    }

    pub async fn get_identifier(&self, identifier: Identifier) -> async_graphql::Result<Identifier>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_identifier(identifier)

        })

    }

    //

    pub async fn get_count(&self, namespace: Option<String>) -> async_graphql::Result<usize>
    {

        self.update_or_default_get_ns(&namespace, |namespace: &Option<String>, locked_ns: std::sync::MutexGuard<Namespace>| {

            Ok(locked_ns.get_count())

        })

    }

    fn get_value_names(&self, namespace: Option<String>) -> async_graphql::Result<Vec<String>>
    {
        
        self.update_or_default_get_ns(&namespace, |namespace: &Option<String>, locked_ns: std::sync::MutexGuard<Namespace>|
        {

            Ok(locked_ns.get_value_names())

        })

    }

    
    fn get_value_names_and_types(&self, namespace: Option<String>) -> async_graphql::Result<std::collections::HashMap<String, Type>>
    {
       
        self.update_or_default_get_ns(&namespace, |namespace: &Option<String>, locked_ns: std::sync::MutexGuard<Namespace>|
        {

            Ok(locked_ns.get_value_names_and_types())

        })
        
    }

    fn get_type(&self, identifier: Identifier) -> async_graphql::Result<Type>
    {
       
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.get_type(identifier)

        })

    }

    //Mutations

    pub async fn create_namespace(&self, name: String) -> async_graphql::Result<UnitValue>
    {

        let res = self.namespaces.insert(name, Mtx::new(Namespace::new()));

        if let Err(_err) = res
        {

            //Cache the Mutex?

            return Namespace_already_exists();

        }

        Ok(UnitValue::new())

    }

    pub async fn drop_namespace(&self, name: String) -> async_graphql::Result<UnitValue>
    {

        let res = self.namespaces.remove(&name);

        if let None = res
        {

            return no_namespace_found_with_provided_name();

        }

        Ok(UnitValue::new())

    }

    //NamespaceSettersMut

    pub async fn create_stored_object(&self, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>| {

            locked_ns.create_stored_object(identifier, the_type)

        })

    }

    pub async fn create_stored_object_mut(&self, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>| {

            locked_ns.create_stored_object_mut(identifier, the_type)

        })

    }

    pub async fn create_generic_stored_object(&self, identifier: Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>
    {

        feature_not_implemented()

    }

    pub async fn create_generic_stored_object_mut(&self, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        feature_not_implemented()

    }

    pub async fn uop(&self, identifier: Identifier, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.uop(identifier, op)

        })
        
    }

    pub async fn bop(&self, identifier: Identifier, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.bop(identifier, op, right_side)

        })

    }

    pub async fn bop_self(&self, identifier: Identifier, op: Bop) -> async_graphql::Result<NumericOrBool>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.bop_self(identifier, op)

        })

    }

    pub async fn set_bool(&self, identifier: Identifier, input: bool) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_bool(identifier, input)

        })
       
    }

    pub async fn set_char(&self, identifier: Identifier, input: char) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_char(identifier, input)

        })
        
    }

    pub async fn set_f32(&self, identifier: Identifier, input: f32) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_f32(identifier, input)

        })

    }

    pub async fn set_f64(&self, identifier: Identifier, input: f64) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_f64(identifier, input)

        })

    }

    pub async fn set_i8(&self, identifier: Identifier, input: i8) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_i8(identifier, input)

        })

    }

    pub async fn set_i16(&self, identifier: Identifier, input: i16) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_i16(identifier, input)

        })

    }

    pub async fn set_i32(&self, identifier: Identifier, input: i32) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_i32(identifier, input)

        })

    }

    pub async fn set_i64(&self, identifier: Identifier, input: i64) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_i64(identifier, input)

        })

    }

    pub async fn set_i128(&self, identifier: Identifier, input: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_i128(identifier, input)

        })

    }

    pub async fn set_isize(&self, identifier: Identifier, input: isize) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_isize(identifier, input)

        })

    }

    pub async fn set_u8(&self, identifier: Identifier, input: u8) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_u8(identifier, input)

        })

    }

    pub async fn set_u16(&self, identifier: Identifier, input: u16) -> async_graphql::Result<UnitValue>
    {
       
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_u16(identifier, input)

        })

    }

    pub async fn set_u32(&self, identifier: Identifier, input: u32) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_u32(identifier, input)

        })

    }

    pub async fn set_u64(&self, identifier: Identifier, input: u64) -> async_graphql::Result<UnitValue>
    {
        
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_u64(identifier, input)

        })
        
    }

    pub async fn set_u128(&self, identifier: Identifier, input: U128Scalar) -> async_graphql::Result<UnitValue>
    {
        
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_u128(identifier, input)

        })

    }

    pub async fn set_unit(&self, identifier: Identifier, input: UnitValue) -> async_graphql::Result<UnitValue>
    {
        
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_unit(identifier, input)

        })

    }

    pub async fn set_usize(&self, identifier: Identifier, input: usize) -> async_graphql::Result<UnitValue>
    {
        
        self.update_or_default_ns(&identifier, |identifier: &Identifier, mut locked_ns: MtxGuard<Namespace>|
        {

            locked_ns.set_usize(identifier, input)

        })

    }

    pub async fn set_string(&self, identifier: Identifier, input: String) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns_param(&identifier, input, |identifier, mut locked_ns, input| //: MtxGuard<Namespace> : &Identifier
        {

            locked_ns.set_string(identifier, input)

        })

    }

    pub async fn set_identifier(&self, identifier: Identifier, input: Identifier) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns_param(&identifier, input, |identifier, mut locked_ns, input|
        {

            locked_ns.set_identifier(identifier, input)

        })

    }

    pub async fn set_value(&self, identifier: Identifier, input: AnyInputObject) -> async_graphql::Result<UnitValue>
    {

        self.update_or_default_ns_param(&identifier, input, |identifier, mut locked_ns, input|
        {

            locked_ns.set_value(&identifier, input)

        })

    }

}







