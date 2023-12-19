use std::sync::Arc;

use async_graphql::Object;

use crate::types::{OkValue, get_ok_value_str};

#[derive(Default)]
pub struct MiscellaneousQuery;

#[Object]
impl MiscellaneousQuery
{

    async fn test(&self) -> &'static str
    {

        get_ok_value_str()

    }

    //Experimentation

    /*
    async fn get_arc_str(&self) -> Arc<str>
    {

        let test_string = "test 123 etc".to_string();

        let test_arc_str: Arc<str> = test_string.into();

        test_arc_str

    }

    async fn set_arc_str(&self, _input: Arc<str>) -> bool
    {

        true

    }

    async fn get_arc_i32(&self) -> Arc<[i32]>
    {

        let test_123 = vec![1, 2, 3];

        let test_arc_123: Arc<[i32]> = test_123.into();

        test_arc_123

    }

    async fn get_arc_i8(&self) -> Arc<[i8]>
    {

        let test_123 = vec![1, 2, 3];

        let test_arc_123: Arc<[i8]> = test_123.into();

        test_arc_123

    }

    async fn set_arc_i8(&self, _input: Arc<[i8]>) -> bool
    {

        true

    }
    */
    
    //JSONObject

    /*
    async fn hash_map(&self) -> HashMap<u32, String>
    {

        HashMap::new()

    }
    */

    /*
    async fn can_do_join_handles(&self) -> JoinHandle<async_graphql::Result<&'static str>>
    {

        task::spawn(async { Ok("Great") })

        //Ok("Great")

        //Nope

    }
    */

    //Some experimentation

    /*
    async fn can_do_join_handles(&self) -> JoinHandle<&'static str>
    {

        task::spawn(async { "Great" })

        //Ok("Great")

        //Nope

    }
    */

    /*
    async fn arc_string(&self, arst: Arc<String>) -> Arc<String>
    {

        arst

    }
    
    async fn vec_arc_string(&self, vec_arst: Vec<Arc<String>>) -> Vec<Arc<String>>
    {

        vec_arst

    }

    async fn vec_vec_u8(&self, vec_vec_no: Vec<Vec<u8>>) -> Vec<Vec<u8>>
    {

        vec_vec_no

    }

    async fn vec_vec_arc_string(&self, vec_vec_arst: Vec<Vec<Arc<String>>>) -> Vec<Vec<Arc<String>>>
    {

        vec_vec_arst

    }

    async fn vec_vec_arc_u8(&self, vec_vec_arc_no: Vec<Vec<Arc<u8>>>) -> Vec<Vec<Arc<u8>>>
    {

        vec_vec_arc_no

    }
    */

}



