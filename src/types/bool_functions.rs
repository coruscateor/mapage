
pub fn not(value: &mut bool) -> async_graphql::Result<bool>
{

    *value = !*value;

    Ok(*value)

}

/*
pub fn get_bool_not_fn<K>() -> dyn FnOnce(&K, &mut bool) -> async_graphql::Result<bool>
{

    |&key, value| {

        Ok(not(value))

    }

}
*/

pub fn bit_and(value: &mut bool, right_side: bool) -> async_graphql::Result<bool>
{

    *value = *value & right_side;

    Ok(*value)

}

pub fn bit_or(value: &mut bool, right_side: bool) -> async_graphql::Result<bool>
{

    *value = *value | right_side;

    Ok(*value)

}

pub fn bit_xor(value: &mut bool, right_side: bool) -> async_graphql::Result<bool>
{

    *value = *value ^ right_side;

    Ok(*value)

}

pub fn bit_and_self(value: &mut bool) -> async_graphql::Result<bool>
{

    *value = *value & *value;

    Ok(*value)

}

pub fn bit_or_self(value: &mut bool) -> async_graphql::Result<bool>
{

    *value = *value | *value;

    Ok(*value)

}

pub fn bit_xor_self(value: &mut bool) -> async_graphql::Result<bool>
{

    *value = *value ^ *value;

    Ok(*value)

}


