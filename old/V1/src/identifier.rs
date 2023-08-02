use async_graphql::*; //InputObject;

//https://async-graphql.github.io/async-graphql/en/define_simple_object.html

#[derive(InputObject, SimpleObject, Clone, Default)]
#[graphql(input_name = "IdentifierInput")]
pub struct Identifier
{

    name: String,
    namespace: Option<String>

}

impl Identifier
{

    pub fn new(name: String, namespace: Option<String>) -> Self
    {

        Self
        {

            name,
            namespace

        }
        
    }

    pub fn get_name_ref(&self) -> &String
    {

        &self.name

    }

    pub fn get_namespace_ref(&self) -> &Option<String>
    {

        &self.namespace

    }

    pub fn get_name_mut(&mut self) -> &mut String
    {

        &mut self.name

    }

    pub fn get_namespace_mut(&mut self) -> &mut Option<String>
    {

        &mut self.namespace

    }

    pub fn set_name(&mut self, value: String)
    {

        self.name = value;

    }

    pub fn set_namespace(&mut self, value: String)
    {

        self.namespace = Some(value);

    }

}