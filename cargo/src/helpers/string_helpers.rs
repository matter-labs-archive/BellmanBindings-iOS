use std::error::Error;
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

pub fn ptr_to_string(cstr: *const c_char) -> Result<String, Box<Error>> {
    let cstr = unsafe { CStr::from_ptr(cstr) };
    let _str = match cstr.to_str() {
        Err(error) => {
            return Result::Err(Box::new(error))
        },
        Ok(string) => string,
    };
    Ok(_str.to_string())
}
