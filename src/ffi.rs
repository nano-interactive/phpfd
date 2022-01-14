use std::os::raw::{c_char, c_void};

use ext_php_rs::types::Zval;
use php_all_sys::php80::_php_stream;

extern "C" {
    pub fn php_stream_cast(
        stream: *mut _php_stream,
        castas: u32,
        ret: *mut *mut c_void,
        show_err: i32,
    ) -> i32;

    pub fn zend_fetch_resource_ex(
        res: *mut Zval,
        resource_type_name: *const c_char,
        resource_type: i32,
    ) -> *mut c_void;
}
