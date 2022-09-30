use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_spawn(port_: i64) {
    wire_spawn_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_poll(raw: u64) -> support::WireSyncReturnStruct {
    wire_poll_impl(raw)
}

#[no_mangle]
pub extern "C" fn wire_drop_future(raw: u64) -> support::WireSyncReturnStruct {
    wire_drop_future_impl(raw)
}

// Section: allocate functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}