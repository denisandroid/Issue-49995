#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
    println_str(cstr!("cluWorld!!!"));
    //CSTR "cluWorld!!!"
    //CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12
    
    println_str(cstr!(b"cluWorld!!!"));
    //CSTR "cluWorld!!!"
    //CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12
    
    println_str(cstr!(b'A'));
    //CSTR "A"
    //CArray [65, 0] 2
}


fn println_str(cstr: &CStr) {
    println!("CSTR {:?}", cstr);
    
    let cstr_array = cstr.to_bytes_with_nul();
    println!("CArray {:?} {}", cstr_array, cstr_array.len());
    println!();
}


