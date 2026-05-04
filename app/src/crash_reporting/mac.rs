//! macOS Cocoa crash reporting stubs. Sentry integration has been removed.

pub fn init_cocoa_sentry() {}

pub fn uninit_cocoa_sentry() {}

pub fn crash() {}

pub fn set_user_id(_user_id: &str) {}

pub fn forward_breadcrumb(_message: &str, _category: &str, _level: &str) {}

pub fn set_tag(_key: &str, _value: &str) {}
