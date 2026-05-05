use std::{path::PathBuf, sync::Arc};

use warpui::{Entity, ModelContext, SingletonEntity};

use crate::auth::auth_state::AuthState;

/// Constructs the cloud preferences syncer.  In the local-first build this is
/// a no-op stub: settings are already persisted to `settings.toml` by the
/// `SettingsManager` and no cloud round-trip is needed.
pub fn initialize_cloud_preferences_syncer(
    _toml_file_path: PathBuf,
    _startup_toml_parse_error: Option<&str>,
    ctx: &mut ModelContext<CloudPreferencesSyncer>,
) -> CloudPreferencesSyncer {
    CloudPreferencesSyncer::new(ctx)
}

/// No-op singleton that previously synced user preferences to the Warp cloud.
/// Kept as a registered singleton so that subscribers (e.g. `OneTimeModalModel`)
/// that wait for `InitialLoadCompleted` still receive the event after login.
pub struct CloudPreferencesSyncer;

/// Events emitted by the CloudPreferencesSyncer.
#[derive(Debug)]
pub enum CloudPreferencesSyncerEvent {
    /// Fired (deferred) after `handle_user_fetched` is called so that
    /// subscribers waiting for the initial-load signal are notified.
    InitialLoadCompleted,
    /// No longer emitted; kept for source-compatibility with existing match arms.
    Updated { key: String, value: String },
}

impl CloudPreferencesSyncer {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    /// Called after a user is fetched.  Emits `InitialLoadCompleted` on the
    /// next event-loop tick so that any subscriptions registered during the
    /// `AuthComplete` handler receive the event.
    pub fn handle_user_fetched(
        &mut self,
        _auth_state: Arc<AuthState>,
        ctx: &mut ModelContext<Self>,
    ) {
        ctx.spawn(async {}, |_, _, ctx| {
            ctx.emit(CloudPreferencesSyncerEvent::InitialLoadCompleted);
        });
    }

    /// Previously pushed local preference values to the cloud.  Now a no-op.
    pub fn maybe_sync_local_prefs_to_cloud(
        &mut self,
        _keys_to_sync: Vec<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

impl Entity for CloudPreferencesSyncer {
    type Event = CloudPreferencesSyncerEvent;
}

impl SingletonEntity for CloudPreferencesSyncer {}
