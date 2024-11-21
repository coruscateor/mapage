use std::{vec::Vec, sync::Arc, mem};

use core::mem::size_of;

use anyhow::{Result, Error};

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

//Constructors - Arc

pub fn arc_vec_new<T>() -> Arc<Vec<T>>
{
    
    Arc::new(Vec::new())

}

pub fn arc_vec_with_capacity<T>(capacity: usize) -> Arc<Vec<T>>
{
    
    Arc::new(Vec::with_capacity(capacity))

}

pub fn arc_vec_with_no_capacity<T>() -> Arc<Vec<T>>
{
    
    Arc::new(Vec::with_capacity(0))

}

//Queries

//capacity

pub fn get_vec_capacity_fn<T>() -> impl FnOnce(&Vec<T>) -> Result<usize>
    //where F:  //, F
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.capacity())

    }

}

fn will_go_over_capacity<T>(col: &Vec<T>, additional: usize) -> Option<Result<&'static str>>
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

        return Some(Err(Error::msg("Error: new capacity exeeds isize::MAX bytes of Vec contained type.")));

    }

    //Some(Ok(get_ok_value_str()))

    None

}

//len

pub fn get_vec_len_fn<T>() -> impl FnOnce(&Vec<T>) -> Result<usize>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.len())

    }

}

//is_empty

pub fn get_vec_is_empty_fn<T>() -> impl FnOnce(&Vec<T>) -> Result<bool>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.is_empty())

    }

}

//is_ascii

/*
pub fn get_vec_is_ascii_fn<T>() -> impl Fn(&Vec<T>) -> Result<bool>
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.is_ascii())

    }

}
*/

//first

pub fn get_vec_first_fn<T>() -> impl FnOnce(&Vec<T>) -> Result<Option<T>>
    where T : Clone
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.first().cloned())

    }

}

//last

pub fn get_vec_last_fn<T>() -> impl FnOnce(&Vec<T>) -> Result<Option<T>>
    where T : Clone
{
    
    |col: &Vec<T>|
    {

        Result::Ok(col.last().cloned())

    }

}

//contains

pub fn get_vec_contains_fn<'a, T>(x: &'a T) -> impl FnOnce(&'a Vec<T>) -> Result<bool>
    where T : PartialEq
{
    
    move |col: &Vec<T>|
    {

        Result::Ok(col.contains(x))

    }

}

//binary_search

pub fn get_vec_binary_search_fn<T>(x: T) -> impl FnOnce(&Vec<T>) -> Result<usize>
    where T : Ord
{
    
    move |col: &Vec<T>|
    {

        match col.binary_search(&x)
        {

            Result::Ok(val) =>
            {

                Result::Ok(val)

            },
            Result::Err(err) =>
            {

                let mut ag_err = Error::msg("Binary search error");
                
                Result::Err(ag_err)

            }

        }

    }

}

//binary_search - with channel

//repeat - repeat_into

//

fn is_out_of_index<T, R>(col: &Vec<T>, index: usize) -> Option<Result<R>>
{

    let len = col.len();

    if len == 0
    {

        return Some(Err(Error::msg("Error: Vec is empty.")));

    }

    let max_index = len - 1;

    if index <= max_index
    {

        return None;

    }

    Some(Err(Error::msg("Error: The provided index is out of bounds.")))

}

//

fn is_out_of_index_only<T, R>(col: &Vec<T>, index: usize) -> Option<Result<R>>
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

    Some(Err(Error::msg("Error: The provided index is out of bounds.")))

}

fn is_out_of_index_must_have_len<T>(col: &Vec<T>, index: usize) -> Option<Result<&'static str>>
{

    let len = col.len();

    if len == 0
    {

        return Some(Err(Error::msg("Error: The provided index is out of bounds.")));

    }

    if index < len
    {

        return None;

    }

    Some(Err(Error::msg("Error: The provided index is out of bounds.")))

}

//index - get_at_index

pub fn get_vec_index_fn<T>(index: usize) -> impl FnOnce(&Vec<T>) -> Result<T>
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

//index_mut - set_at_index

pub fn get_vec_index_mut_fn<T>(index: usize, value: T) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
    where T : Clone
{
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index::<T, &'static str>(col, index)
        {

            return val;

        }

        col[index] = value;

        Result::Ok(get_ok_value_str())

    }

}

//reserve

pub fn get_vec_reserve_fn<T>(additional: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_reserve_exact_fn<T>(additional: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_try_reserve_fn<T>(additional: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Err(_err) = col.try_reserve(additional)
        {

            return Result::Err(Error::msg("Error"));

        }

        Result::Ok(get_ok_value_str())

    }

}

//try_reserve_exact

pub fn get_vec_try_reserve_exact_fn<T>(additional: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        if let Err(_err) = col.try_reserve_exact(additional)
        {

            return Result::Err(Error::msg("Error"));

        }
        
        Result::Ok(get_ok_value_str())

    }

}

//shrink_to_fit

pub fn get_vec_shrink_to_fit_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        col.shrink_to_fit();

        Result::Ok(get_ok_value_str())

    }

}

//shrink_to

pub fn get_vec_shrink_to_fn<T>(min_capacity: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{

    move |col: &mut Vec<T>|
    {

        col.shrink_to(min_capacity);

        Result::Ok(get_ok_value_str())

    }

}

//truncate

pub fn get_vec_truncate_fn<T>(len: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_insert_fn<T>(index: usize, element: T) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_push_fn<T>(value: T) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_pop_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<Option<T>>
{
    
    move |col: &mut Vec<T>|
    {

        let poped = col.pop();

        Result::Ok(poped)

    }

}

//append

/*
pub fn get_vec_append_fn<'a, T>(other: &'a mut Vec<T>) -> impl FnOnce(&'a mut Vec<T>) -> Result<&'static str>
{
    
    let mv = move |col: &'a mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, other.len())
        {

            return val;

        }

        col.append(other);

        mem::drop(other);

        Result::Ok(get_ok_value_str())

    };

    mem::drop(other);

    mv

}
*/

pub fn get_vec_append_fn<T>(mut other: Vec<T>) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = will_go_over_capacity(col, other.len())
        {

            return val; //(val, other);

        }

        col.append(&mut other);

        Result::Ok(get_ok_value_str())

        //(Result::Ok(get_ok_value_str()), other)

    }

}

//get_vec_append_fn - with channel

//append_risky

//clear

pub fn get_vec_clear_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        col.clear();

        Result::Ok(get_ok_value_str())

    }

}

//split_off

pub fn get_vec_split_off_fn<T>(at: usize) -> impl FnOnce(&mut Vec<T>) -> Result<Vec<T>>
{
    
    move |col: &mut Vec<T>|
    {

        if let Some(val) = is_out_of_index_only(col, at)
        {

            return val;

        }

        Result::Ok(col.split_off(at))

    }

}

//resize

pub fn get_vec_resize_fn<T>(new_len: usize, value: T) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_dedup_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
    where T : PartialEq
{
    
    move |col: &mut Vec<T>|
    {

        col.dedup();

        Result::Ok(get_ok_value_str())

    }

}

//sort_unstable

pub fn get_vec_sort_unstable_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
    where T : Ord
{
    
    move |col: &mut Vec<T>|
    {

        col.sort_unstable();

        Result::Ok(get_ok_value_str())

    }

}

//rotate_left

pub fn get_vec_rotate_left_fn<T>(mid: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_rotate_right_fn<T>(mid: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_fill_fn<T>(value: T) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_sort_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
    where T : Ord
{
    
    move |col: &mut Vec<T>|
    {

        col.sort();

        Result::Ok(get_ok_value_str())

    }

}

//swap

pub fn get_vec_swap_fn<T>(a: usize, b: usize) -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
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

pub fn get_vec_reverse_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<&'static str>
{
    
    move |col: &mut Vec<T>|
    {

        col.reverse();

        Result::Ok(get_ok_value_str())

    }

}

//index_mut

//

pub fn get_vec_retrieve_contents_fn<T>() -> impl FnOnce(&mut Vec<T>) -> Result<Vec<T>>
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



