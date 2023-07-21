#[repr(C)]
pub struct BoolVec {
    _layout: [u64; 3],
}

#[allow(unused_doc_comments)]
/// cbindgen:ignore
extern "C" {
    pub fn bool_vec_new() -> *mut BoolVec;
    pub fn bool_vec_free(bool_vec: *mut BoolVec);
    pub fn bool_vec_push(bool_vec: *mut BoolVec, value: bool);
    pub fn bool_vec_pop(bool_vec: *mut BoolVec) -> bool;
    pub fn bool_vec_get(bool_vec: *mut BoolVec, index: usize) -> bool;
}

#[no_mangle]
pub extern "C" fn test() {
    let a = unsafe { bool_vec_new() };
    unsafe { bool_vec_push(a, true) };
    println!("{:?}", unsafe { bool_vec_get(a, 0) });
    unsafe { bool_vec_free(a) };
}
