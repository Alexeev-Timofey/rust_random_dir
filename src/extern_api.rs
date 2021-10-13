use crate::{generate_file as r_generate_file};
use crate::config::*;
use std::error::Error;
use std::path::Path;
use std::ffi::{CStr, CString};

#[derive(Clone, Copy)]
#[repr(C)]
enum RngConfigType {
    CYCLE_GEN,
    RANGE_GEN,
    BIT_TRAIN_GEN,
    STRING_GEN,
}

#[derive(Clone, Copy)]
#[repr(C)]
enum ItemConfigType {
    FILE_CONFIG,
    DIRECTORY_CONFIG,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct vector_data<'a> {
    value_p: & 'a [u8],
    value_len: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct vector_vector_data<'a> {
    value_p: & 'a [vector_data<'a>],
    value_len: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RngConifg_CycleGen<'a> {
    value: vector_data<'a>,
    length: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RngConfig_RangeGen<'a> {
    value: vector_vector_data<'a>,
    seed: Option<& 'a vector_data<'a>>,
    length: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RngConfig_BitTrain<'a> {
    probability: f64,
    seed: Option<& 'a vector_data<'a>>,
    length: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RngConfig_StringGen<'a> {
    value: & 'a [u8],
}

#[derive(Clone, Copy)]
#[repr(C)]
union RngConfigContent<'a> {
    cg: RngConifg_CycleGen<'a>,
    rg: RngConfig_RangeGen<'a>,
    bt: RngConfig_BitTrain<'a>,
    sg: RngConfig_StringGen<'a>,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CRngConfig<'a> {
    rng_type: RngConfigType,
    content: RngConfigContent<'a>,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ItemConfig_FileConfig<'a> {
    name: CRngConfig<'a>,
    rand_gen: CRngConfig<'a>,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct vector_items<'a> {
    items_p: & 'a [CRngConfig<'a>],
    items_len: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ItemConfig_DirectoryConfig<'a> {
    name: CRngConfig<'a>,
    items: vector_items<'a>,
}

#[repr(C)]
union ItemConfigContent<'a> {
    fc: ItemConfig_FileConfig<'a>,
    dc: ItemConfig_DirectoryConfig<'a>,
}

#[repr(C)]
struct CItemConfig<'a> {
    item_type: ItemConfigType,
    content: ItemConfigContent<'a>,
}

fn convert_conf(input: & CItemConfig) -> Result<ItemConfig, Box<dyn Error>> {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn generate_file(conf: & CItemConfig, dir_path: &[u8]) -> i32 {
    let rs_conf: ItemConfig = convert_conf(conf).expect("Can't convert config to rust types");
    let rs_dir_path_cstr: & CStr = CStr::from_bytes_with_nul(dir_path).expect("Can't convert directory path to rust string");
    let rs_dir_path: String = CString::from(rs_dir_path_cstr).into_string().expect("Can't convert CStr to String");
    r_generate_file(& rs_conf, (& (* rs_dir_path)) as & Path).map_or_else(|_|{1i32}, |_|{0i32})
}
