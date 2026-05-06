//! Crash reporting stubs. All Sentry integration has been removed for local-first operation.

use std::borrow::Cow;

use warpui::{AppContext, SingletonEntity};

use crate::antivirus::AntivirusInfo;
use crate::auth::UserUid;
use crate::features::FeatureFlag;
use crate::settings::PrivacySettings;
use warpui::rendering::GPUDeviceInfo;
use warpui::windowing::WindowManager;

/// Initializes the crash reporting subsystem. Always returns false (disabled).
pub(crate) fn init(ctx: &mut AppContext) -> bool {
    if !FeatureFlag::CrashReporting.is_enabled() {
        log::info!("Crash reporting FeatureFlag is disabled.");
        return false;
    }

    // Subscribe to window manager and antivirus events so event subscriptions
    // don't break other parts of the app that depend on these models being observed.
    let window_manager = WindowManager::handle(ctx);
    ctx.subscribe_to_model(&window_manager, |_, _event, _| {});

    let antivirus_info = AntivirusInfo::handle(ctx);
    ctx.subscribe_to_model(&antivirus_info, |_, _event, _| {});

    let privacy_settings = PrivacySettings::handle(ctx);
    ctx.subscribe_to_model(&privacy_settings, |_, _event, _| {});

    false
}

/// No-op stub for uninitializing crash reporting.
pub fn uninit_sentry() {}

/// No-op stub for setting a crash reporting tag.
pub(crate) fn set_tag<'a, 'b>(_key: impl Into<Cow<'a, str>>, _value: impl Into<Cow<'b, str>>) {}

/// No-op stub for setting GPU device info.
pub(crate) fn set_gpu_device_info(_gpu_device_info: GPUDeviceInfo) {}

/// No-op stub for setting antivirus info.
pub fn set_antivirus_info(_antivirus_info: &AntivirusInfo) {}

/// No-op stub for setting the client type tag.
pub fn set_client_type_tag(_client_id: &str) {}

/// No-op stub for initializing Cocoa Sentry.
pub fn init_cocoa_sentry() {}

/// No-op stub for uninitializing Cocoa Sentry.
pub fn uninit_cocoa_sentry() {}

/// No-op stub for triggering a test crash.
pub fn crash() {}

/// No-op stub for setting the user id.
pub fn set_user_id(_user_id: UserUid, _email: Option<String>, _ctx: &mut AppContext) {}
