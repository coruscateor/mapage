use serde::{Deserialize, Serialize};

use crate::types::{SupportedType, TypeInstance};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Command
{

    pub id: Option<u32>,
    pub command: String, //Optional when namespaces get added.
    pub type_name: Option<SupportedType>,
    pub params: Option<Vec<Option<TypeInstance>>>
    
}
