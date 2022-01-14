mod fd;
mod ffi;

use ext_php_rs::{
    builders::ModuleBuilder,
    exception::{PhpException, PhpResult},
    php_function, php_module,
    types::Zval,
};

///
/// Fetches the UNIX File Descriptor from any `php resource` that is file
/// @param resource $resource
///
/// @return int
#[php_function]
pub fn resource_fd(resource: &mut Zval) -> PhpResult<i64> {
    if resource.resource().is_none() {
        return Err(PhpException::default("Invalid Resource".to_string()));
    }

    let fd = unsafe { fd::fetch_fd(resource) };

    if fd == 0 {
        Err(PhpException::default(
            "Cannot obtain Unix FD from resource".to_string(),
        ))
    } else {
        Ok(fd)
    }
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
