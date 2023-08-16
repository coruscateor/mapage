//Build Configuration API

use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub struct AllCfgs
{

    store_aml: bool,
    sub_store_aml: bool,
    scc_hashmap_namespaces: bool,
    dashmap_namespaces: bool,
    all_types: bool,
    bool_type: bool,
    char_type: bool,
    f32_type: bool,
    f64_type: bool,
    i8_type: bool,
    i16_type: bool,
    i32_type: bool,
    i64_type: bool,
    i128_type: bool,
    isize_type: bool,
    string_type: bool, //String
    u8_type: bool,
    u16_type: bool,
    u32_type: bool,
    u64_type: bool,
    u128_type: bool,
    usize_type: bool,
    whatever_type: bool, //Whatever
    selected_type_type: bool, //SelectedType
    selected_type_io_type: bool, //  SelectedTypeIO

    //Keys Types:

    all_key_types_string: bool, //all_key_types_String
    all_key_types_arc_string: bool, //all_key_types_Arc_String

    //bool keys

    bool_key_type_string: bool, //bool_key_type_String
    bool_key_type_arc_string: bool, //bool_key_type_Arc_String

    //char keys

    char_key_type_string: bool, //char_key_type_String
    char_key_type_arc_string: bool, //char_key_type_Arc_String

    //f32 keys

    f32_key_type_string: bool, //f32_key_type_String
    f32_key_type_arc_string: bool, //f32_key_type_Arc_String

    //f64 keys

    f64_key_type_string: bool, //f64_key_type_String
    f64_key_type_arc_string: bool, //f64_key_type_Arc_String:

    //i8 keys

    i8_key_type_string: bool, //i8_key_type_String
    i8_key_type_arc_string: bool, //i8_key_type_Arc_String

    //i16 keys

    i16_key_type_string: bool, //i16_key_type_String
    i16_key_type_arc_string: bool, //i16_key_type_Arc_String

    //i32 keys

    i32_key_type_string: bool, //i32_key_type_String
    i32_key_type_arc_string: bool, //i32_key_type_Arc_String

    //i64 keys

    i64_key_type_string: bool, //i64_key_type_String
    i64_key_type_arc_string: bool, //i64_key_type_Arc_String

    //i128 keys

    i128_key_type_string: bool, //i128_key_type_String
    i128_key_type_arc_string: bool, //i128_key_type_Arc_String

    //isize keys

    isize_key_type_string: bool, //isize_key_type_String
    isize_key_type_arc_string: bool, //isize_key_type_Arc_String

    //SelectedType keys

    selected_rype_key_type_string: bool, //SelectedType_key_type_String
    selected_type_key_type_arc_string: bool, //SelectedType_key_type_Arc_String

    //String keys

    string_key_type_string: bool, //String_key_type_String
    string_key_type_arc_string: bool, //String_key_type_Arc_String

    //u8 keys

    u8_key_type_string: bool, //u8_key_type_String
    u8_key_type_arc_string: bool, //u8_key_type_Arc_String

    //u16 keys

    u16_key_type_string: bool, //u16_key_type_String
    u16_key_type_arc_string: bool, //u16_key_type_Arc_String

    //u32 keys

    u32_key_type_string: bool, //u32_key_type_String
    u32_key_type_arc_string: bool, //u32_key_type_Arc_String

    //u64 keys

    u64_key_type_string: bool, //u64_key_type_String
    u64_key_type_arc_string: bool, //u64_key_type_Arc_String

    //u128 keys

    u128_key_type_string: bool, //u128_key_type_String
    u128_key_type_arc_string: bool, //u128_key_type_Arc_String

    //usize keys

    usize_key_type_string: bool, //usize_key_type_String
    usize_key_type_arc_string: bool, //usize_key_type_Arc_String

    //Whatever keys

    whatever_key_type_string: bool, //Whatever_key_type_String
    whatever_key_type_arc_string: bool //Whatever_key_type_Arc_String

}

impl AllCfgs
{

    pub fn new() -> Self
    {

        Self
        {

            store_aml: cfg!(feature = "store_aml"),
            sub_store_aml:cfg!(feature = "sub_store_aml"),
            scc_hashmap_namespaces: cfg!(feature = "scc_hashmap_namespaces"),
            dashmap_namespaces: cfg!(feature = "dashmap_namespaces"),
            all_types: cfg!(feature = "all_types"),
            bool_type: cfg!(feature = "bool"),
            char_type: cfg!(feature = "char"),
            f32_type: cfg!(feature = "f32"),
            f64_type: cfg!(feature = "f64"),
            i8_type: cfg!(feature = "i8"),
            i16_type: cfg!(feature = "i16"),
            i32_type: cfg!(feature = "i32"),
            i64_type: cfg!(feature = "i64"),
            i128_type: cfg!(feature = "i128"),
            isize_type: cfg!(feature = "isize"),
            string_type: cfg!(feature = "String"),
            u8_type: cfg!(feature = "u8"),
            u16_type: cfg!(feature = "u16"),
            u32_type: cfg!(feature = "u32"),
            u64_type: cfg!(feature = "u64"),
            u128_type: cfg!(feature = "u128"),
            usize_type: cfg!(feature = "usize"),
            whatever_type: cfg!(feature = "Whatever"),
            selected_type_type: cfg!(feature = "SelectedType"),
            selected_type_io_type: cfg!(feature = "SelectedTypeIO"),

            //Keys Types:

            all_key_types_string: cfg!(feature = "all_key_types_String"), //all_key_types_String
            all_key_types_arc_string: cfg!(feature = "all_key_types_Arc_String"), //all_key_types_Arc_String

            //bool keys

            bool_key_type_string: cfg!(feature = "bool_key_type_String"), //bool_key_type_String
            bool_key_type_arc_string: cfg!(feature = "bool_key_type_Arc_String"), //bool_key_type_Arc_String

            //char keys

            char_key_type_string: cfg!(feature = "char_key_type_String"), //char_key_type_String
            char_key_type_arc_string: cfg!(feature = "char_key_type_Arc_String"), //char_key_type_Arc_String

            //f32 keys

            f32_key_type_string: cfg!(feature = "f32_key_type_String"), //f32_key_type_String
            f32_key_type_arc_string: cfg!(feature = "f32_key_type_Arc_String"), //f32_key_type_Arc_String

            //f64 keys

            f64_key_type_string: cfg!(feature = "f64_key_type_String"), //f64_key_type_String
            f64_key_type_arc_string: cfg!(feature = "f64_key_type_Arc_String"), //f64_key_type_Arc_String

            //i8 keys

            i8_key_type_string: cfg!(feature = "i8_key_type_String"), //i8_key_type_String
            i8_key_type_arc_string: cfg!(feature = "i8_key_type_Arc_String"), //i8_key_type_Arc_String

            //i16 keys

            i16_key_type_string: cfg!(feature = "i16_key_type_String"), //i16_key_type_String
            i16_key_type_arc_string: cfg!(feature = "i16_key_type_Arc_String"), //i16_key_type_Arc_String

            //i32 keys

            i32_key_type_string: cfg!(feature = "i32_key_type_String"), //i32_key_type_String
            i32_key_type_arc_string: cfg!(feature = "i32_key_type_Arc_String"), //i32_key_type_Arc_String

            //i64 keys

            i64_key_type_string: cfg!(feature = "i64_key_type_String"), //i64_key_type_String
            i64_key_type_arc_string: cfg!(feature = "i64_key_type_Arc_String"), //i64_key_type_Arc_String

            //i128 keys

            i128_key_type_string: cfg!(feature = "i128_key_type_String"), //i128_key_type_String
            i128_key_type_arc_string: cfg!(feature = "i128_key_type_Arc_String"), //i128_key_type_Arc_String

            //isize keys

            isize_key_type_string: cfg!(feature = "isize_key_type_String"), //isize_key_type_String
            isize_key_type_arc_string: cfg!(feature = "isize_key_type_Arc_String"), //isize_key_type_Arc_String

            //SelectedType keys

            selected_rype_key_type_string: cfg!(feature = "SelectedType_key_type_String"), //SelectedType_key_type_String
            selected_type_key_type_arc_string: cfg!(feature = "SelectedType_key_type_Arc_String"), //SelectedType_key_type_Arc_String
        
            //String keys
        
            string_key_type_string: cfg!(feature = "String_key_type_String"), //String_key_type_String
            string_key_type_arc_string: cfg!(feature = "String_key_type_Arc_String"), //String_key_type_Arc_String

            //u8 keys

            u8_key_type_string: cfg!(feature = "u8_key_type_String"), //u8_key_type_String
            u8_key_type_arc_string: cfg!(feature = "u8_key_type_Arc_String"), //u8_key_type_Arc_String

            //u16 keys

            u16_key_type_string: cfg!(feature = "u16_key_type_String"), //u16_key_type_String
            u16_key_type_arc_string: cfg!(feature = "u16_key_type_Arc_String"), //u16_key_type_Arc_String

            //u32 keys

            u32_key_type_string: cfg!(feature = "u32_key_type_String"), //u32_key_type_String
            u32_key_type_arc_string: cfg!(feature = "u32_key_type_Arc_String"), //u32_key_type_Arc_String

            //u64 keys

            u64_key_type_string: cfg!(feature = "u64_key_type_String"), //u64_key_type_String
            u64_key_type_arc_string: cfg!(feature = "u64_key_type_Arc_String"), //u64_key_type_Arc_String

            //u128 keys

            u128_key_type_string: cfg!(feature = "u128_key_type_String"), //u128_key_type_String
            u128_key_type_arc_string: cfg!(feature = "u128_key_type_Arc_String"), //u128_key_type_Arc_String

            //usize keys

            usize_key_type_string: cfg!(feature = "usize_key_type_String"), //usize_key_type_String
            usize_key_type_arc_string: cfg!(feature = "usize_key_type_Arc_String"), //usize_key_type_Arc_String

            //Whatever keys

            whatever_key_type_string: cfg!(feature = "Whatever_key_type_String"), //Whatever_key_type_String
            whatever_key_type_arc_string: cfg!(feature = "Whatever_key_type_Arc_String") //Whatever_key_type_Arc_String

        }        

    }



}

#[derive(Default)]
pub struct Cfgs;

#[Object]
impl Cfgs
{

    async fn get_all_cfgs(&self) -> AllCfgs
    {
        
        AllCfgs::new()

    }

    async fn get_store_aml_cfg(&self) -> bool
    {

        cfg!(feature = "store_aml")

    }

    async fn get_sub_store_aml_cfg(&self) -> bool
    {
        
        cfg!(feature = "sub_store_aml")

    }

    async fn get_scc_hashmap_namespaces_cfg(&self) -> bool
    {

        cfg!(feature = "scc_hashmap_namespaces")

    }

    async fn get_dashmap_namespaces_cfg(&self) -> bool
    {

        cfg!(feature = "dashmap_namespaces")

    }

    async fn get_all_types_cfg(&self) -> bool
    {

        cfg!(feature = "all_types")

    }

    async fn get_bool_type_cfg(&self) -> bool
    {

        cfg!(feature = "bool")

    }

    async fn get_char_type_cfg(&self) -> bool
    {

        cfg!(feature = "char")

    }

    async fn get_f32_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "f32")

    }

    async fn get_f64_type_cfg(&self) -> bool
    { 
        
        cfg!(feature = "f64")

    }

    async fn get_i8_type_cfg(&self) -> bool
    {

        cfg!(feature = "i8")

    }

    async fn get_i16_type_cfg(&self) -> bool
    {

        cfg!(feature = "i16")

    }

    async fn get_i32_type_cfg(&self) -> bool
    {

        cfg!(feature = "i32")

    }

    async fn get_i64_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "i64")

    }

    async fn get_i128_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "i128")

    }

    async fn get_isize_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "isize")

    }

    async fn get_string_type_cfg(&self) -> bool
    {

        cfg!(feature = "String")

    }

    async fn get_u8_type_cfg(&self) -> bool
    {

        cfg!(feature = "u8")

    }

    async fn get_u16_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "u16")

    }

    async fn get_u32_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "u32")

    }

    async fn get_u64_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "u64")

    }

    async fn get_u128_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "u128")

    }

    async fn get_usize_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "usize")

    }

    async fn get_whatever_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "Whatever")

    }

    async fn get_selected_type_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "SelectedType")

    }

    async fn get_selected_type_io_type_cfg(&self) -> bool
    {
        
        cfg!(feature = "SelectedTypeIO")

    }

    //Keys Types:

    async fn get_all_key_types_string_cfg(&self) -> bool
    {

        cfg!(feature = "all_key_types_String")

    }

    async fn get_all_key_types_arc_string_cfg(&self) -> bool
    {

        cfg!(feature = "all_key_types_Arc_String")

    }

    //bool keys

    async fn get_bool_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "bool_key_type_String")

    }

    async fn get_bool_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "bool_key_type_Arc_String")

    }

    //char keys

    async fn get_char_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "char_key_type_String")

    }
    
    async fn get_char_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "char_key_type_Arc_String")

    }

    //f32 keys

    async fn get_f32_key_type_string_cfg(&self) -> bool
    {
        
         cfg!(feature = "f32_key_type_String")

    }

    async fn get_f32_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "f32_key_type_Arc_String")

    }

    //f64 keys

    async fn get_f64_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "f64_key_type_String")

    }

    async fn get_f64_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "f64_key_type_Arc_String")

    }

    //i8 keys

    async fn get_i8_key_type_string_cfg(&self) -> bool
    {

        cfg!(feature = "i8_key_type_String")

    }

    async fn get_i8_key_type_arc_string_cfg(&self) -> bool
    {

        cfg!(feature = "i8_key_type_Arc_String")

    }

    //i16 keys

    async fn get_i16_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i16_key_type_String")

    }

    async fn get_i16_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i16_key_type_Arc_String")

    }

    //i32 keys

    async fn get_i32_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i32_key_type_String")

    }

    async fn get_i32_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i32_key_type_Arc_String")

    }

    //i64 keys

    async fn get_i64_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i64_key_type_String")

    }

    async fn get_i64_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i64_key_type_Arc_String")

    }

    //i128 keys

    async fn get_i128_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i128_key_type_String")

    }

    async fn get_i128_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "i128_key_type_Arc_String")

    }

    //isize keys

    async fn get_isize_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "isize_key_type_String")

    }

    async fn get_isize_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "isize_key_type_Arc_String")

    }

    //SelectedType keys

    async fn get_selected_rype_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "SelectedType_key_type_String")

    }

    async fn get_selected_type_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "SelectedType_key_type_Arc_String")

    }

    //String keys

    async fn get_string_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "String_key_type_String")

    }

    async fn get_string_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "String_key_type_Arc_String")

    }

    //u8 keys

    async fn get_u8_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u8_key_type_String")

    }

    async fn get_u8_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u8_key_type_Arc_String")

    }

    //u16 keys

    async fn get_u16_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u16_key_type_String")

    }

    async fn get_u16_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u16_key_type_Arc_String")

    }

    //u32 keys

    async fn get_u32_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u32_key_type_String")

    }

    async fn get_u32_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u32_key_type_Arc_String")

    }

    //u64 keys

    async fn get_u64_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u64_key_type_String")

    }

    async fn get_u64_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u64_key_type_Arc_String")

    }

    //u128 keys

    async fn get_u128_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u128_key_type_String")

    }


    async fn get_u128_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "u128_key_type_Arc_String")

    }

    //usize keys

    async fn get_usize_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "usize_key_type_String")

    }

    async fn get_uusize_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "usize_key_type_Arc_String")

    }

    //Whatever keys

    async fn get_whatever_key_type_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "Whatever_key_type_String")

    }

    async fn get_whatever_key_type_arc_string_cfg(&self) -> bool
    {
        
        cfg!(feature = "Whatever_key_type_Arc_String")

    }
    
}

