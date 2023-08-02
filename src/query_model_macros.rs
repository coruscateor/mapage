use paste::paste;

#[macro_export]
macro_rules! impl_store_insert
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _insert>](&self, ctx: &Context<'_>,key: String, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                ctx.data_unchecked::<Store>().[<$label _insert>](key, value).await

            }

        }

    }

}

/*
async fn get_u8(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<u8>
{

    //Ok(0)

    ctx.data_unchecked::<Store>().get_u8(identifier).await

}
*/

