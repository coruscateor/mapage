
/*

the trait bound `Vec<Whatever>: From<Vec<InputOneOfWhatever>>` is not satisfied
the following other types implement trait `From<T>`:
  <Vec<u8> as From<tungstenite::protocol::message::Message>>
  <Vec<u8> as From<CString>>
  <Vec<u8> as From<std::string::String>>
  <Vec<u8> as From<&str>>
  <Vec<ServerError> as From<ServerError>>
  <Vec<T> as From<[T; N]>>
  <Vec<T, A> as From<Box<[T], A>>>
  <Vec<T, A> as From<BinaryHeap<T, A>>>
and 4 others
required for `Vec<InputOneOfWhatever>` to implement `Into<Vec<Whatever>>`rustcClick for full compiler diagnostic
core::convert::Into
pub fn into(self) -> T
Converts this type into the (usually inferred) input type.

 */

use super::async_graphql_values::{InputOneOfWhatever, Whatever};

/*
use std::vec::Vec;

impl From<Vec<InputOneOfWhatever>> for Vec<Whatever>
{

    fn from(value: Vec<InputOneOfWhatever>) -> Self
    {


        
    }

}
*/

//Corlib

pub fn into_vec<TFrom, TInto>(mut from_vec: Vec<TFrom>) -> Vec<TInto>
    where //TFrom: From<TInto>,
          TInto: From<TFrom>
{

    let mut to_vec = Vec::with_capacity(from_vec.len());

    for item in from_vec.drain(0..)
    {

        to_vec.push(item.into());

    }

    to_vec

}

