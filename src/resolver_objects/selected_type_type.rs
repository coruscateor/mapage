use async_graphql::{Object, Context, SimpleObject};

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
use crate::types::{sizes::size_of_selected_type, async_graphql_values::*};

use super::StoreType;

use std::collections::{HashMap, HashSet};

use tokio::{task, task::JoinHandle};

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

use paste::paste;

type KeyType = crate::types::keys::SelectedTypeKeyType;

#[derive(Default)]
pub struct SelectedTypeQuery;

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
#[Object]
impl SelectedTypeQuery 
{

    pub async fn selected_type_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<SelectedType>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, read, key)

    }

    pub async fn selected_type_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<SelectedType>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, try_read, key)

    }

    pub async fn selected_type_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, contains, key)

    }

    pub async fn selected_type_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, len)

    }

    pub async fn selected_type_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, is_empty)

    }

    pub async fn selected_type_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, capacity)

    }

    pub async fn selected_type_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "SelectedType")))] //, feature = "SelectedTypeIO"
#[Object]
impl SelectedTypeQuery 
{

    #[graphql(visible = false)]
    pub async fn selected_type_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct SelectedTypeMutation;

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
#[Object]
impl SelectedTypeMutation
{
    
    pub async fn selected_type_insert(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfSelectedType) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_selected_type_namespace_ref, insert, key, value.into())

    }

    pub async fn selected_type_update(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfSelectedType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, update, key, value.into())

    }

    pub async fn selected_type_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfSelectedType) -> Option<SelectedType>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, try_replace, key, value.into())

    }

    pub async fn selected_type_upsert(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfSelectedType) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_selected_type_namespace_ref, upsert, key, value.into())

    }

    pub async fn selected_type_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, remove, key)

    }

    pub async fn selected_type_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<SelectedType>
    {

        call_store_method!(ctx, get_selected_type_namespace_ref, try_retrieve, key)

    }

    pub async fn selected_type_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, clear)

    }

    pub async fn selected_type_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_selected_type_namespace_ref, clear_and_get_len)

    }

    //methods...

}

#[cfg(not(any(feature = "all_types", feature = "SelectedType")))]
#[Object]
impl SelectedTypeMutation
{

    #[graphql(visible = false)]
    pub async fn selected_type_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

/*

//To be possibly re-implemented:

#[derive(Default)]
pub struct SelectedTypeQueryMisc;

#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))]
#[Object]
impl SelectedTypeQueryMisc 
{

    async fn size_of_selected_type(&self) -> usize
    {

        size_of_selected_type()

    }

}

#[cfg(not(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO")))]
#[Object]
impl SelectedTypeQueryMisc
{

    #[graphql(visible = false)]
    pub async fn selected_type_type_query_misc_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

//IO

macro_rules! ast_case_push_result
{

    ($res_vec:ident, $res_cap_minus_one:ident, $ctx_cpy:ident, $namespace_ref_method:ident, $key:ident, $st_case:ident) =>
    {

        paste! {

            //Is this the last or only value to be added to the res_vec?

            if $res_vec.len() == $res_cap_minus_one
            {

                //if so use this thread/task

                match $ctx_cpy.data_unchecked::<StoreType>().$namespace_ref_method().read(&$key).await
                {

                    Ok(res) =>
                    {

                        $res_vec.push(SelectedTypeResult::no_jh(async_graphql::Result::Ok(SelectedType::$st_case([<$st_case Value>]::new(res)))));

                    },
                    Err(err) =>
                    {

                        $res_vec.push(SelectedTypeResult::no_jh(async_graphql::Result::Err(err)));

                    }

                }

                continue;

            }

            //If not spawn a task which gets the value

            let store_clone = $ctx_cpy.data_unchecked::<StoreType>().clone();

            let str = SelectedTypeResult::new(task::spawn(async move
            {

                //async_graphql::Result::Ok(SelectedType::$st_case([<$st_case Value>]::new($ctx_cpy.data_unchecked::<StoreType>().$namespace_ref_method().read(&$key).await?)))

                async_graphql::Result::Ok(SelectedType::$st_case([<$st_case Value>]::new(store_clone.$namespace_ref_method().read(&$key).await?)))

            }));

            $res_vec.push(str);

        }

    }
    
}

#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] 
#[derive(SimpleObject)]
pub struct SelectedTypeResult
{

    #[graphql(skip)]
    jh_opt: Option<JoinHandle<async_graphql::Result<SelectedType>>>,
    result: Option<async_graphql::Result<SelectedType>>

}

#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] 
impl SelectedTypeResult
{

    pub fn new(jh: JoinHandle<async_graphql::Result<SelectedType>>) -> Self
    {

        Self
        {

            jh_opt: Some(jh),
            result: None

        }

    }

    pub fn no_jh(result: async_graphql::Result<SelectedType>) -> Self
    {

        Self
        {

            jh_opt: None,
            result: Some(result)

        }

    }

    pub async fn try_set_result_from_jh(&mut self)
    {

        if let Some(jh) = &mut self.jh_opt
        {

            match jh.await
            {

                Ok(res) =>
                {

                    self.result = Some(res);

                },
                Err(err) =>
                {

                    self.result = Some(async_graphql::Result::Err(err.into()));

                }

            }

        }

    }

}

#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] 
#[derive(SimpleObject)]
pub struct SelectedTypeOptional
{

    #[graphql(skip)]
    jh_opt: Option<JoinHandle<Option<SelectedType>>>,
    result: Option<SelectedType>

}

#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] 
impl SelectedTypeOptional
{

    pub fn new(jh: JoinHandle<Option<SelectedType>>) -> Self
    {

        Self
        {

            jh_opt: Some(jh),
            result: None

        }

    }

    pub fn no_jh(result: Option<SelectedType>) -> Self
    {

        Self
        {

            jh_opt: None,
            result

        }

    }

    pub async fn try_set_result_from_jh(&mut self)
    {

        if let Some(jh) = &mut self.jh_opt
        {

            match jh.await
            {

                Ok(res) =>
                {

                    self.result = res;

                },
                Err(_err) =>
                {

                    //Could set a String or Arc<String> if this is the case

                    self.result = None;

                }

            }

        }

    }

}

macro_rules! ast_case_push_optional
{

    ($res_vec:ident, $res_cap_minus_one:ident, $ctx_cpy:ident, $namespace_ref_method:ident, $key:ident, $st_case:ident) =>
    {
        
        paste! {

            //Is this the last or only value to be added to the res_vec?

            if $res_vec.len() == $res_cap_minus_one
            {

                //if so use this thread/task

                match $ctx_cpy.data_unchecked::<StoreType>().$namespace_ref_method().try_read(&$key).await
                {

                    Some(res) =>
                    {

                        $res_vec.push(SelectedTypeOptional::no_jh(Some(SelectedType::$st_case([<$st_case Value>]::new(res)))));

                    },
                    None =>
                    {

                        $res_vec.push(SelectedTypeOptional::no_jh(None));

                    }

                }

                continue;

            }

            //If not spawn a task which gets the value

            let store_clone = $ctx_cpy.data_unchecked::<StoreType>().clone();

            let str = SelectedTypeOptional::new(task::spawn(async move
            {
    
                let res_opt = store_clone.$namespace_ref_method().try_read(&$key).await;

                if let Some(res) = res_opt
                {

                    return Some(SelectedType::$st_case([<$st_case Value>]::new(res)));

                }

                None
    
            }));

            $res_vec.push(str);

        }

    }
    
}

#[derive(Default)]
pub struct SelectedTypeIOQuery;

#[cfg(any(feature = "all_types", feature = "SelectedTypeIO"))]
#[Object]
impl SelectedTypeIOQuery 
{

    async fn read_selected_type_values(&self, ctx: &Context<'_>, mut kvps: HashMap<String, String>) -> Vec<SelectedTypeResult> //&'static Context<'_>, //HashMap<AvalibleSelectedType, String>) -> Vec<SelectedTypeResult>
    {

        let mut res_vec = Vec::with_capacity(kvps.len());

        let res_cap = res_vec.capacity();

        if res_cap == 0
        {

            return res_vec;

        }

        let res_cap_minus_one = res_cap - 1;

        for item in kvps.drain()
        {

            let key = item.1;

            let ast;

            match AvalibleSelectedType::try_from(item.0)
            {

                Ok(res) => ast = res,
                Err(err) =>
                {

                    res_vec.push(SelectedTypeResult::no_jh(Err(err)));

                    continue;

                }

            }

            let ctx_cpy = ctx;

            match ast
            {

                #[cfg(any(feature = "all_types", feature = "bool"))]
                AvalibleSelectedType::Bool =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_bool_namespace_ref, key, Bool);

                },
                #[cfg(any(feature = "all_types", feature = "Char"))]
                AvalibleSelectedType::Char =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_char_namespace_ref, key, Char);

                },
                #[cfg(any(feature = "all_types", feature = "f32"))]
                AvalibleSelectedType::F32 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_f32_namespace_ref, key, F32);

                },
                #[cfg(any(feature = "all_types", feature = "f64"))]
                AvalibleSelectedType::F64 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_f64_namespace_ref, key, F64);

                },
                #[cfg(any(feature = "all_types", feature = "i8"))]
                AvalibleSelectedType::I8 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_i8_namespace_ref, key, I8);

                },
                #[cfg(any(feature = "all_types", feature = "i16"))]
                AvalibleSelectedType::I16 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_i16_namespace_ref, key, I16);

                },
                #[cfg(any(feature = "all_types", feature = "i32"))]
                AvalibleSelectedType::I32 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_i32_namespace_ref, key, I32);

                },
                #[cfg(any(feature = "all_types", feature = "i64"))]
                AvalibleSelectedType::I64 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_i64_namespace_ref, key, I64);

                },
                #[cfg(any(feature = "all_types", feature = "i128"))]
                AvalibleSelectedType::I128 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_i128_namespace_ref, key, I128);

                },
                #[cfg(any(feature = "all_types", feature = "isize"))]
                AvalibleSelectedType::ISize =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_isize_namespace_ref, key, ISize);

                },
                #[cfg(any(feature = "all_types", feature = "u8"))]
                AvalibleSelectedType::U8 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_u8_namespace_ref, key, U8);

                },
                #[cfg(any(feature = "all_types", feature = "u16"))]
                AvalibleSelectedType::U16 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_u16_namespace_ref, key, U16);

                },
                #[cfg(any(feature = "all_types", feature = "u32"))]
                AvalibleSelectedType::U32 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_u32_namespace_ref, key, U32);

                },
                #[cfg(any(feature = "all_types", feature = "u64"))]
                AvalibleSelectedType::U64 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_u64_namespace_ref, key, U64);

                },
                #[cfg(any(feature = "all_types", feature = "u128"))]
                AvalibleSelectedType::U128 =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_u128_namespace_ref, key, U128);

                },
                #[cfg(any(feature = "all_types", feature = "usize"))]
                AvalibleSelectedType::USize =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_usize_namespace_ref, key, USize);

                },
                #[cfg(any(feature = "all_types", feature = "String"))]
                AvalibleSelectedType::String =>
                {

                    ast_case_push_result!(res_vec, res_cap_minus_one, ctx_cpy, get_string_namespace_ref, key, String);

                }


            }

        }

        for item in res_vec.iter_mut()
        {

            item.try_set_result_from_jh().await;

        }

        res_vec

    }

    async fn try_read_selected_type_values(&self, ctx: &Context<'_>, mut kvps: HashMap<String, String>) -> Vec<SelectedTypeOptional>
    {

        let mut res_vec = Vec::with_capacity(kvps.len());

        let res_cap = res_vec.capacity();

        if res_cap == 0
        {

            return res_vec;

        }

        let res_cap_minus_one = res_cap - 1;

        for item in kvps.drain()
        {

            let key = item.1;

            let ast;

            match AvalibleSelectedType::try_from(item.0)
            {

                Ok(res) => ast = res,
                Err(_err) =>
                {

                    res_vec.push(SelectedTypeOptional::no_jh(None));

                    continue;

                }

            }

            let ctx_cpy = ctx;

            match ast
            {

                #[cfg(any(feature = "all_types", feature = "bool"))]
                AvalibleSelectedType::Bool =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_bool_namespace_ref, key, Bool);

                },
                #[cfg(any(feature = "all_types", feature = "Char"))]
                AvalibleSelectedType::Char =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_char_namespace_ref, key, Char);

                },
                #[cfg(any(feature = "all_types", feature = "f32"))]
                AvalibleSelectedType::F32 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_f32_namespace_ref, key, F32);

                },
                #[cfg(any(feature = "all_types", feature = "f64"))]
                AvalibleSelectedType::F64 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_f64_namespace_ref, key, F64);

                },
                #[cfg(any(feature = "all_types", feature = "i8"))]
                AvalibleSelectedType::I8 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_i8_namespace_ref, key, I8);

                },
                #[cfg(any(feature = "all_types", feature = "i16"))]
                AvalibleSelectedType::I16 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_i16_namespace_ref, key, I16);

                },
                #[cfg(any(feature = "all_types", feature = "i32"))]
                AvalibleSelectedType::I32 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_i32_namespace_ref, key, I32);

                },
                #[cfg(any(feature = "all_types", feature = "i64"))]
                AvalibleSelectedType::I64 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_i64_namespace_ref, key, I64);

                },
                #[cfg(any(feature = "all_types", feature = "i128"))]
                AvalibleSelectedType::I128 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_i128_namespace_ref, key, I128);

                },
                #[cfg(any(feature = "all_types", feature = "isize"))]
                AvalibleSelectedType::ISize =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_isize_namespace_ref, key, ISize);

                },
                #[cfg(any(feature = "all_types", feature = "u8"))]
                AvalibleSelectedType::U8 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_u8_namespace_ref, key, U8);

                },
                #[cfg(any(feature = "all_types", feature = "u16"))]
                AvalibleSelectedType::U16 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_u16_namespace_ref, key, U16);

                },
                #[cfg(any(feature = "all_types", feature = "u32"))]
                AvalibleSelectedType::U32 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_u32_namespace_ref, key, U32);

                },
                #[cfg(any(feature = "all_types", feature = "u64"))]
                AvalibleSelectedType::U64 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_u64_namespace_ref, key, U64);

                },
                #[cfg(any(feature = "all_types", feature = "u128"))]
                AvalibleSelectedType::U128 =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_u128_namespace_ref, key, U128);

                },
                #[cfg(any(feature = "all_types", feature = "usize"))]
                AvalibleSelectedType::USize =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_usize_namespace_ref, key, USize);

                },
                #[cfg(any(feature = "all_types", feature = "String"))]
                AvalibleSelectedType::String =>
                {

                    ast_case_push_optional!(res_vec, res_cap_minus_one, ctx_cpy, get_string_namespace_ref, key, String);

                }


            }

        }

        for item in res_vec.iter_mut()
        {

            item.try_set_result_from_jh().await;

        }

        res_vec

    }

}

#[cfg(not(any(feature = "all_types", feature = "SelectedTypeIO")))]
#[Object]
impl SelectedTypeIOQuery
{

    #[graphql(visible = false)]
    pub async fn selected_type_io_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
*/
