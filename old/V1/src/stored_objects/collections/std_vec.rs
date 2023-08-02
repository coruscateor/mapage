use crate::
{
    
    value::*,
    all_types::*,
    stored_object::*,
    async_graphql_value_containers::*,
    uniary_operations::*,
    binary_operations::*

};

//memrs_type::Type, stored_object::Object,

//pub struct Bool 

//pub type StdVec = TypedValue<Vec<Box<dyn StoredObject>>>;

/*
pub struct StdVec
{

    value: Vec<Box<dyn StoredObject>>,
    contained_type: Type

}

impl StdVec
{

    pub fn new(contained_type: Type) -> Self
    {

        //TypedValue::typed_new(Vec::new(), contained_type)
        
        Self
        {
            
            value: Vec::new(),
            contained_type

        }
        
        //Vec::with_capacity(20),

        //TypedValue::typed_new(Vec::with_capacity(20), Type::new_generic_single("Vec", &gtype))

    }

}

impl StoredObject for StdVec
{

    fn get_type(&self) -> Type //, all_types: &AllTypes
    {
        
        //Type::new("Vec")

        //self.type_clone()

        //Type::Vec { params_count: Some(1) }

        //self.get_contained_type()

        //Type::Vec { params: all_types.get_rcd_type(&self.contained_type) } //get_all_types().get_rcd_type(&self.contained_type) }

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        todo!()

    }

    /*
    fn is_mutable(&self) -> bool
    {
        
        false

    }

    fn get_type_ref(&self) -> &Type
    {
        
        self.type_ref()

    }
    
    fn op(op) -> res
    {

        add,
        remove
        etc...

    }

    */

}
*/

/*
pub struct MutStdVec
{

    value: Vec<Box<dyn StoredObject>>,
    contained_type: Type

}

impl MutStdVec
{

    pub fn new(contained_type: Type) -> Self
    {
        
        Self
        {
            
            value: Vec::new(),
            contained_type

        }

    }

}

impl StoredObject for MutStdVec
{

    fn get_type(&self, all_types: &AllTypes) -> Type
    {

        Type::Vec { params: all_types.get_rcd_type(&self.contained_type) } //get_all_types().get_rcd_type(&self.contained_type) }

    }
    
    /* 
    fn uop_mut(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        todo!()

    }

    fn bop_mut(&self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool>
    {

        todo!()

    }

    fn is_mutable(&self) -> bool
    {
        
        true

    }
    */

    /*

    fn get_type_ref(&self) -> &Type
    {
        
        self.type_ref()

    }
    
    fn op(op) -> res
    {

        add,
        remove
        etc...

    }

    */

}
*/