use std::{vec::Vec};

use core::mem::size_of;

use async_graphql::{ErrorExtensionValues, ErrorExtensions};

use crate::types::get_ok_value_str;

//https://doc.rust-lang.org/std/vec/struct.Vec.html

//Constructors

pub fn vec_new<T>() -> Vec<T>
{
    
    Vec::new()

}

pub fn vec_with_capacity<T>(capacity: usize) -> Vec<T>
{
    
    Vec::with_capacity(capacity)

}

pub fn vec_with_no_capacity<T>() -> Vec<T>
{
    
    Vec::with_capacity(0)

}

//Queries

//capacity

pub fn get_vec_capacity_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<usize>
    //where F:  //, F
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.capacity())

    }

}

fn will_go_over_capacity<T>(col: &Vec<T>, additional: usize) -> Option<async_graphql::Result<&'static str>>
{

    //Should be in Corlib

    let cap_diff = col.capacity() - col.len();

    if cap_diff >= additional
    {

        //Capacity will not change

        return None;

    }

    let new_cap = col.capacity() + additional;

    let size_of_value_in_bytes = size_of::<T>();

    let new_cap_bytes = new_cap * size_of_value_in_bytes;

    let max_bytes = isize::MAX as usize * size_of_value_in_bytes;

    if new_cap_bytes > max_bytes
    {

        return Some(Err(async_graphql::Error::new("Error: new capacity exeeds isize::MAX bytes of Vec contained type.")));

    }

    //Some(Ok(get_ok_value_str()))

    None

}

//len

pub fn get_vec_len_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<usize>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.len())

    }

}

//is_empty

pub fn get_vec_is_empty_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<bool>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.is_empty())

    }

}

//is_ascii

/*
pub fn get_vec_is_ascii_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<bool>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.is_ascii())

    }

}
*/

//first

pub fn get_vec_first_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<Option<T>>
    where T : Clone
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.first().cloned())

    }

}

//last

pub fn get_vec_last_fn<T>() -> impl Fn(&Vec<T>) -> async_graphql::Result<Option<T>>
    where T : Clone
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.last().cloned())

    }

}

//contains

pub fn get_vec_contains_fn<'a, T>(x: &'a T) -> impl Fn(&'a Vec<T>) -> async_graphql::Result<bool>
    where T : PartialEq
{
    
    move |col: &Vec<T>|
    {

        Result::Ok(col.contains(x))

    }

}

//binary_search

pub fn get_vec_binary_search_fn<'a, T>(x: &'a T) -> impl Fn(&'a Vec<T>) -> async_graphql::Result<usize>
    where T : Ord
{
    
    move |col: &Vec<T>|
    {

        match col.binary_search(x)
        {

            Result::Ok(val) =>
            {

                async_graphql::Result::Ok(val)

            },
            Result::Err(err) =>
            {

                let mut ag_err = async_graphql::Error::new("Binary search error");

                ag_err = ag_err.extend_with(|_, extv| {

                    extv.set("index", err);

                });

                async_graphql::Result::Err(ag_err)

            }

        }

    }

}

//repeat - repeat_into

//

fn is_out_of_index<T>(col: &Vec<T>, index: usize) -> Option<async_graphql::Result<T>>
{

    let len = col.len();

    if len == 0
    {

        return Some(Err(async_graphql::Error::new("Error: Vec is empty.")));

    }

    let max_index = len - 1;

    if index <= max_index
    {

        return None;

    }

    Some(Err(async_graphql::Error::new("Error: The provided index is out of bounds.")))

}

//

fn is_out_of_index_only<T>(col: &Vec<T>, index: usize) -> Option<async_graphql::Result<&'static str>>
{

    let len = col.len();

    if len == 0
    {

        return None;

    }

    if index < len
    {

        return None;

    }

    Some(Err(async_graphql::Error::new("Error: The provided index is out of bounds.")))

}

fn is_out_of_index_must_have_len<T>(col: &Vec<T>, index: usize) -> Option<async_graphql::Result<&'static str>>
{

    let len = col.len();

    if len == 0
    {

        return Some(Err(async_graphql::Error::new("Error: The provided index is out of bounds.")));

    }

    if index < len
    {

        return None;

    }

    Some(Err(async_graphql::Error::new("Error: The provided index is out of bounds.")))

}

//index - get_at_index

pub fn get_vec_get_at_index_fn<T>(index: usize) -> impl Fn(&Vec<T>) -> async_graphql::Result<T>
    where T : Clone
{
    
    move |col: &Vec<T>|
    {

        if let Some(val) = is_out_of_index(col, index)
        {

            return val;

        }

        Result::Ok(col[index].clone())

    }

}

//index_risky



//Mutations

//reserve

pub fn get_vec_reserve_fn<T>(additional: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, additional)
        {

            return val;

        }

        col.reserve(additional);

        Result::Ok(get_ok_value_str())

    }

}

//reserve_risky

//reserve_exact

pub fn get_vec_reserve_exact_fn<T>(additional: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, additional)
        {

            return val;

        }

        col.reserve_exact(additional);

        Result::Ok(get_ok_value_str())

    }

}

//reserve_exact_risky

//try_reserve

pub fn get_vec_try_reserve_fn<T>(additional: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Err(_err) = col.try_reserve(additional)
        {

            return Result::Err(async_graphql::Error::new("Error"));

        }

        Result::Ok(get_ok_value_str())

    }

}

//try_reserve_exact

pub fn get_vec_try_reserve_exact_fn<T>(additional: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Err(_err) = col.try_reserve_exact(additional)
        {

            return Result::Err(async_graphql::Error::new("Error"));

        }
        
        Result::Ok(get_ok_value_str())

    }

}

//shrink_to_fit

pub fn get_vec_shrink_to_fit_fn<T>() -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        col.shrink_to_fit();

        Result::Ok(get_ok_value_str())

    }

}

//shrink_to

pub fn get_vec_shrink_to_fn<T>(min_capacity: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        col.shrink_to(min_capacity);

        Result::Ok(get_ok_value_str())

    }

}

//truncate

pub fn get_vec_truncate_fn<T>(len: usize) -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        col.truncate(len);

        Result::Ok(get_ok_value_str())

    }

}

//swap_remove

//pub fn swap_remove(&mut self, index: usize) -> T

//insert

pub fn get_vec_insert_fn<T>(index: usize, element: T) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    //let mut my_index = Some(index);

    let mut my_element = Some(element);

    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_only(col, index)
        {

            return val;

        }

        //let index = my_index.take().unwrap();

        let element = my_element.take().unwrap();

        col.insert(index, element);

        Result::Ok(get_ok_value_str())

    }

}


//remove

//push

pub fn get_vec_push_fn<T>(value: T) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
{

    let mut my_value = Some(value);
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, 1)
        {

            return val;

        }

        let value_to_push = my_value.take().unwrap();

        col.push(value_to_push);

        Result::Ok(get_ok_value_str())

    }

}

//push_risky

//pop

pub fn get_vec_pop_fn<T>() -> impl Fn(&mut Vec<T>) -> async_graphql::Result<Option<T>>
{
    
    move |col: &mut Vec<T>|
    {

        let poped = col.pop();

        Result::Ok(poped)

    }

}

//append

pub fn get_vec_append_fn<'a, T>(other: &'a mut Vec<T>) -> impl FnMut(&'a mut Vec<T>) -> async_graphql::Result<&'static str>
{
    
    move |col: &'a mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, other.len())
        {

            return val;

        }

        col.append(other);

        Result::Ok(get_ok_value_str())

    }

}

//append_risky

//clear

pub async fn clear_fn<T>() -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        col.clear();

        Result::Ok(get_ok_value_str())

    }

}

//split_off

pub async fn split_off_fn<T>(at: usize) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_only(col, at)
        {

            return val;

        }

        col.split_off(at);

        Result::Ok(get_ok_value_str())

    }

}

//resize

pub async fn resize_fn<T>(new_len: usize, value: T) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Clone
{

    let mut my_value = Some(value);
    
    move |col: &mut Vec<T>|
    {

        let value = my_value.take().unwrap();

        col.resize(new_len, value);

        Result::Ok(get_ok_value_str())

    }

}

//dedup

pub async fn dedup_fn<T>() -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : PartialEq
{
    
    move |col: &mut Vec<T>|
    {

        col.dedup();

        Result::Ok(get_ok_value_str())

    }

}

//sort_unstable

pub async fn sort_unstable_fn<T>() -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Ord
{
    
    move |col: &mut Vec<T>|
    {

        col.sort_unstable();

        Result::Ok(get_ok_value_str())

    }

}

//rotate_left

pub async fn rotate_left_fn<T>(mid: usize) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Ord
{

    //let mut my_mid = Some(mid);
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_only(col, mid)
        {

            return val;

        }

        //let mid = my_mid.take().unwrap();

        col.rotate_left(mid);

        Result::Ok(get_ok_value_str())

    }

}

//rotate_left_risky

//rotate_right

pub async fn rotate_right_fn<T>(mid: usize) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Ord
{

    //let mut my_mid = Some(mid);
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_only(col, mid)
        {

            return val;

        }

        //let mid = my_mid.take().unwrap();

        col.rotate_right(mid);

        Result::Ok(get_ok_value_str())

    }

}

//rotate_right_risky

//fill

pub async fn fill_fn<T>(value: T) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Clone
{

    let mut my_value = Some(value);
    
    move |col: &mut Vec<T>|
    {

        let value = my_value.take().unwrap();

        col.fill(value);

        Result::Ok(get_ok_value_str())

    }

}

//sort

pub async fn sort_fn<T>() -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Ord
{
    
    move |col: &mut Vec<T>|
    {

        col.sort();

        Result::Ok(get_ok_value_str())

    }

}

//swap

pub async fn swap_fn<T>(a: usize, b: usize) -> impl FnMut(&mut Vec<T>) -> async_graphql::Result<&'static str>
    where T : Ord
{

    //let mut my_a = Some(value);
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_must_have_len(col, a)
        {
    
            return val;
    
        }
    
        if let Some(val) = is_out_of_index_must_have_len(col, b)
        {
    
            return val;
    
        }

        col.swap(a, b);

        Result::Ok(get_ok_value_str())

    }

}

//swap_risky

//reverse

pub async fn reverse_fn<T>() -> impl Fn(&mut Vec<T>) -> async_graphql::Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        col.reverse();

        Result::Ok(get_ok_value_str())

    }

}

//index_mut

//

pub async fn retrieve_contents_fn<T>() -> impl Fn(&mut Vec<T>) -> async_graphql::Result<Vec<T>>
{

    move |col: &mut Vec<T>|
    {

        let mut contents = Vec::with_capacity(col.len());

        for item in col.drain(0..)
        {

            contents.push(item);

        }

        Result::Ok(contents)

    }

}

/*

//make_ascii_uppercase

//make_ascii_lowercase

*/



