mod btree;
#[macro_use]
mod misc;
mod blk;
mod os;
mod session;
mod util;
#[macro_use]
mod error;
#[macro_use]
mod internal;
#[macro_use]
mod global;
mod test;
mod meta;
#[macro_use]
mod stats;
mod context;
mod scheme;
mod cursor;
mod file;
mod support;
mod evict;
mod gcc;
mod common;
mod disk;

#[no_mangle]
pub extern "C" fn Java_foam_posei_Poseidon_addNumbers(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    a: i32,
    b: i32,
) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn Java_foam_posei_Poseidon_mulNumbers(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    a: i32,
    b: i32,
) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
