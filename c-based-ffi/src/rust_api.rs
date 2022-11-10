type Callback = unsafe extern "C" fn(n: u32);

#[no_mangle]
pub extern "C" fn r_add_cb(a: u32, b: u32, cb: Callback) -> u32 {
    unsafe {
        cb(a + b);
    }
    a + b
}