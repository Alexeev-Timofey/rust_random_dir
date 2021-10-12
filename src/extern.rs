#[repr(C)]
enum RngConfigType {
    CYCLE_GEN,
    RANGE_GEN,
    BIT_TRAIN_GEN,
    STRING_GEN,
}

#[repr(C)]
enum ItemConfigType {
    FILE_CONFIG,
    DIRECTORY_CONFIG,
}

#[repr(C)]
struct vector_data {
    value_p: * const u8,
    value_len: usize,
}

#[repr(C)]
struct vector_vector_data {
    value_p: * const vector_data,
    value_len: usize,
}

#[repr(C)]
struct RngConifg_CycleGen {
    value: vector_data,
    length: usize,
}

#[repr(C)]
struct RngConfig_RangeGen {
    value: vector_vector_data,
    seed: Option<& vector_data>,
    length: usize,
}

#[repr(C)]
struct RngConfig_BitTrain {
    probability: f64,
    seed: Option<& vector_data>,
    length: usize,
}

#[repr(C)]
struct RngConfig_StringGen {
    value: * const u8,
}

#[repr(C)]
union RngConfigContent {
    cg: RngConifg_CycleGen,
    rg: RngConfig_RangeGen,
    bt: RngConfig_BitTrain,
    sg: RngConfig_StringGen,
}

#[repr(C)]
struct CRngConfig {
    rng_type: RngConfigType,
    content: RngConfigContent,
}

#[repr(C)]
struct ItemConfig_FileConfig {
    name: CRngConfig,
    rand_gen: CRngConfig,
}

#[repr(C)]
struct vector_items {
    items_p: * const CRngConfig,
    items_len: usize,
}

#[repr(C)]
struct ItemConfig_DirectoryConfig {
    name: CRngConfig,
    items: vector_items,
}

#[repr(C)]
union ItemConfigContent {
    fc: ItemConfig_FileConfig,
    dc: ItemConfig_DirectoryConfig,
}

#[repr(C)]
struct CItemConfig {
    item_type: ItemConfigType,
    content: ItemConfigContent,
}

#[no_mangle]
pub extern "C" fn generate_file(conf: & CItemConfig, dir_path: * const u8) -> i32 {
    unimplemented!()
}
