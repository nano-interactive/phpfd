use ext_php_rs::types::Zval;
use php_all_sys::php80::{
    php_file_le_stream, php_stream, PHP_STREAM_AS_FD_FOR_SELECT, PHP_STREAM_CAST_INTERNAL,
};

use crate::ffi::{php_stream_cast, zend_fetch_resource_ex};

pub(crate) unsafe fn fetch_fd(resource: &mut Zval) -> i64 {
    let stream = zend_fetch_resource_ex(
        resource as *mut Zval,
        std::ptr::null(),
        php_file_le_stream(),
    ) as *mut php_stream;

    let mut fd: i64 = 0;
    let ret = &mut fd as *mut i64;

    let result = php_stream_cast(
        stream,
        PHP_STREAM_AS_FD_FOR_SELECT | PHP_STREAM_CAST_INTERNAL,
        ret as *mut *mut std::ffi::c_void,
        1,
    );

    if result != 0 {
        return 0;
    } else {
        fd
    }
}
