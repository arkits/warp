# Local-First Warp: Cleanup Plan

Goal: strip all Warp account and cloud dependencies so the terminal works fully offline and without login. Where a cloud feature has a useful local equivalent (Drive, AI, settings), keep the UX and wire it to local storage or BYO keys. Where there is no local equivalent (team management, pair programming, session sharing), delete the code outright.

Recent work already done: Sentry crash reporting removed, referral system removed, Slack/Sign Up/Upgrade removed from settings dropdown.

Phase 1a progress:
- Shared-session feature flags have been removed from app startup registration and from `warp_features`: `CreatingSharedSessions`, `ViewingSharedSessions`, `SessionSharingAcls`, `SharedSessionWriteToLongRunningCommands`, `AgentSharedSessions`, and `HOARemoteControl`.
- Session-sharing server experiment handling has been removed from the app's `ServerExperiment` enum and conversion now rejects those legacy GraphQL experiment values.
- User-visible session-sharing entry points have been removed or made inert: terminal command-palette bindings, pane-header menu items, Drive menu item, `/remote-control`, root shared-session actions, and `warp://shared_session` URI parsing.
- The session-sharing server URL override has been removed from CLI args/env parsing, channel config, and child-process environment propagation.
- The original shared-session implementation files have been deleted, with a temporary no-op compatibility layer still present under `app/src/terminal/shared_session/` and `app/src/terminal/view/shared_session/` while remaining call sites are unwound.
- **Recent:** Unwound additional call sites and removed dead code:
  - Deleted `app/src/terminal/view/shared_session/` (completely unused adapter module).
  - Deleted unused submodules from `app/src/terminal/shared_session/`: `participant_avatar_view`, `render_util`, `role_change_modal`, `viewer/`.
  - Removed `SharedSessionBanners` and all related banner state/rendering code from `terminal/view.rs` and `terminal/block_list_element.rs`.
  - Removed `PresenceManager`, `presence_avatars`, and `text_selection_color` references from `terminal/block_list_element.rs`.
  - Removed shared-session size update logic: deleted `SharerSizeChanged` and `ViewerSizeReported` from `SizeUpdateReason`, removed `for_shared_session_update`, `for_viewer_size_report`, `maybe_report_viewer_terminal_size`, `force_report_viewer_terminal_size`, and `natural_rows`/`natural_cols` from `SizeUpdate`.
  - Removed `send_write_to_pty_events_for_shared_session` from `terminal/model/terminal_model.rs` and `terminal/view.rs`.
  - Removed `shared_session_presence_manager` from `TerminalView` and its usage in `ai/blocklist/block/view_impl.rs`.
  - Simplified `workspace/view.rs`: `is_shared_session_viewer_focused`, `is_readonly_shared_session_active`, and shared-session tab-bar checks now return `false` without referencing the shared session module.
  - Removed `copy_shared_session_link` helper and `SessionPermissionsManager` subscription from `terminal/universal_developer_input.rs`.
  - **COMPLETED:** The entire `app/src/terminal/shared_session/` compatibility layer has been deleted (all 9 remaining files).
  - **COMPLETED:** The entire `app/src/terminal/view/shared_session/` adapter module has been deleted.
  - Removed `SharedSessionActionSource`, `SharedSessionScrollbackType`, and all shared-session action variants from `TerminalAction`, `ContextMenuAction`, `InputEvent`, `UseAgentToolbarEvent`, and `WorkspaceAction` enums.
  - Removed all shared-session match arms and handlers across `terminal/view.rs`, `terminal/input.rs`, `workspace/view.rs`, `workspace/action.rs`, `terminal/view/use_agent_footer/mod.rs`, and `terminal/view/action.rs`.
  - Removed `shared_session_response_initiator` and all `participant_id` shared-session plumbing from `ai/blocklist/controller.rs` and `ai/blocklist/controller/slash_command.rs`.
  - Removed `shared_session_state` module from `ai/blocklist/controller/`.
  - Removed `shared_session_status` field usage from `ai/blocklist/block/view_impl/output.rs`.
  - Removed `is_shared_session_viewer()` method and all call sites from `ai/blocklist/action_model/execute.rs`.
  - Removed `shared_session_status`, `set_shared_session_status`, and `is_shared_session_viewer` methods from `terminal/model/terminal_model.rs`.
  - Removed `is_reader()` and `is_viewer()` checks from `terminal/universal_developer_input.rs`, `terminal/alt_screen/alt_screen_element.rs`, `terminal/profile_model_selector.rs`, `workspace/view/wasm_view.rs`, and `ai/blocklist/agent_view/agent_message_bar.rs`.
  - Removed all shared-session-related test code from `workspace/view_test.rs`, `pane_group/mod_tests.rs`, `terminal/input_test.rs`, `test_util/terminal.rs`, `drive/index_test.rs`, `pane_group/pane/view/header/mod_test.rs`, and `ai/ambient_agents/spawn_tests.rs`.
  - Code compiles with zero errors (warnings only).
  - **Result:** `grep -r 'crate::terminal::shared_session'` and `grep -r 'terminal::shared_session'` return zero matches in `app/src/`.

Phase 2a progress:
- Replaced `CloudPreferencesSyncer` with a no-op stub that emits `InitialLoadCompleted` on the next event-loop tick after `handle_user_fetched` is called. All cloud-to-local and local-to-cloud sync methods are no-ops. Settings continue to be persisted locally by `SettingsManager` via `settings.toml`.
- Deleted `app/src/settings/cloud_preferences_syncer_tests.rs`.
- Fixed `root_view.rs`: `AuthComplete + LoginSlide` now applies `pending_post_auth_onboarding_settings` directly instead of waiting for `CloudPreferencesSyncerEvent::InitialLoadCompleted`. The subscription to the syncer remains so `OneTimeModalModel` still receives the event.
- Removed the TOML parse-error extraction in `lib.rs` (it was only used to gate the broken-file guard in the syncer's startup hash logic, which no longer runs).
- Code compiles with zero errors after this phase (`cargo check --package warp`; warnings only).

Phase 1c progress:
- **COMPLETED:** Deleted `app/src/billing/` and `app/src/pricing/mod.rs`, and removed their module declarations from `app/src/lib.rs`.
- Removed `PricingInfoModel` singleton registration from app startup and test setup helpers.
- Removed pricing subscriptions and plan-price lookups from onboarding, teams settings, billing/usage views, buy-credits UI, auto-reload UI, and plan/capacity modals.
- Replaced addon credit option lists with empty local data so deleted pricing metadata is no longer required.
- Stopped ingesting `pricing_info` from workspace metadata into a deleted pricing singleton.
- Removed the shared-object creation denied event path that opened the deleted billing modal.
- Code compiles with zero errors after this phase (`cargo check --package warp`; warnings only).

Phase 2b progress:
- Added `LocalObjectClient` (implements `ObjectClient`): `fetch_changed_objects` returns `Ok(InitialLoadResponse::default())` so the UpdateManager completes initial load; all write operations return errors that the SyncQueue handles gracefully without retrying. Wired into `SyncQueue`, `UpdateManager`, and `Listener` in `lib.rs` — Drive objects persist locally via SQLite, cloud sync attempts fail silently.
- Deleted `app/src/drive/cloud_action_confirmation_dialog.rs` (no call sites outside `mod.rs`).
- Deleted `app/src/cloud_object/grab_edit_access_modal.rs` and removed all references from `notebook.rs`, `active_notebook_data.rs`, `notebook_tests.rs`. In local mode the user is always the sole editor.
- Deleted `app/src/drive/sharing/dialog/` (2315 lines of sharing-dialog UI and ACL logic). Removed `SharingDialog` field and `share_dialog_open_for` state from `DriveIndex`, `ConversationListView`, and the conversation-list item renderer. Made `toggle_share_dialog` a no-op. Removed `drive::sharing::dialog::init(ctx)` from app startup.
- The `drive/sharing/mod.rs` and `style.rs` files remain — they export `ContentEditability`, `SharingAccessLevel`, `ShareableObject`, and extension traits still referenced elsewhere. Full cleanup in Phase 5/6.
- Code compiles with zero errors (`cargo check --package warp`; warnings only).

Phase 2c progress:
- Added `cfg!(feature = "local")` early-return guard to `fetch_ambient_agent_tasks_and_cloud_convo_metadata` in `agent_conversations_model.rs`. When running with `--features local`, no cloud AI task/conversation-metadata fetches are attempted; conversations already stored in local SQLite continue to load normally.
- `persisted_workspace.rs` is workspace-LSP tracking (not cloud AI sync), so no change needed there.
- AI memory (`FeatureFlag::AIMemories`) is cloud-backed; left for a later phase as it requires its own stub.
- Code compiles with zero errors (`cargo check --package warp --features local`).

Phase 4a progress:
- Added `cfg!(feature = "local")` early-return to both `refresh_authed_models` and `refresh_public_models` in `app/src/ai/llms.rs`. In local mode, the server is never queried for model availability; `LLMPreferences` starts from the cached models (or `ModelsByFeature::default()` which includes the "auto" model). BYO API key model list is unaffected.
- `DisableReason::RequiresUpgrade` and `AtCapacity` variants are left in place; they are never used when the server fetch is skipped.
- Code compiles with zero errors (`cargo check --package warp --features local`).

Phase 3a/3b progress:
- Added `local = ["skip_login"]` Cargo feature to `app/Cargo.toml`. Building with `--features local` produces a local-first binary.
- Extended `auth_state.rs`: added `should_use_local_user()` (returns `cfg!(feature = "local")`); `should_use_test_user()` already covers it because `local` implies `skip_login`.
- Added `get_or_create_local_user_id()` in `anonymous_id.rs`: generates a stable `local:<USER>:<uuid>` UID on first run and persists it in user preferences.
- Added `User::local(uid)` in `user.rs`: populates display name from `$USER`, leaves email empty, marks the user as onboarded with no Firebase fields.
- Updated `auth_state.rs::initialize()`: uses `User::local()` instead of `User::test()` when `should_use_local_user()` is true; credentials continue to be set to `Credentials::Test` via the existing `#[cfg(feature = "skip_login")]` gate (implied by `local`).
- Added a comment in `oss.rs` documenting `cargo build --bin warp-oss --features local`.
- Code compiles with zero errors under both default and `--features local` (`cargo check --package warp && cargo check --package warp --features local`).

Phase 1b progress:
- Removed the Teams settings page from settings registration and navigation.
- Deleted the orphaned Teams settings implementation files: `app/src/settings_view/teams_page.rs` and `app/src/settings_view/tab_menu.rs`.
- Removed Teams settings deep links and root-view actions; legacy `warp://settings/teams` / team settings links now route to Account settings instead of opening team management.
- Removed team-management telemetry variants that were only emitted by the deleted Teams page.
- Removed the menu binding/custom action for “Open Team Settings”.
- Removed Drive create/join team sections and their event/action plumbing.
- Deleted `app/src/server/server_api/team.rs` and removed the `TeamClient` from `ServerApi`, `UserWorkspaces`, and app startup wiring.
- Replaced `TeamUpdateManager` with a local-first no-op compatibility model that only updates the selected workspace locally.
- Stopped fetching remote workspace metadata through the team API; workspace metadata refreshes now return an empty local response.
- Removed discoverable/joinable team state, GraphQL discoverable-team conversion, create-team response plumbing, and server-team conversion into runtime workspace state.
- Deleted `app/src/workspaces/team.rs`; retained the few legacy workspace/team cache shapes inside `workspace.rs` while the remaining workspace model is simplified.
- Stopped writing team rows, workspace-team rows, team settings, and team members from workspace metadata into SQLite. Existing team tables are left as legacy read/migration compatibility for now.
- Removed team-derived managed secret configs and the hardcoded Uber team CLI-agent special case.
- Code compiles with zero errors after this slice (`cargo check --package warp`; warnings only).

---

## Guiding Principle

Work in self-contained phases. Each phase should leave the app compiling and running. Never leave dead code behind — when removing a feature, remove its UI entry points, models, server API client code, and feature flags together. Where a model provides data that the rest of the app reads (e.g. `UserWorkspaces`), replace it with a local stub before deleting the cloud implementation.

---

## Phase 1 — Delete Outright: Team, Pair Programming, Session Sharing

These features have no local analogue and can be removed in their entirety. Start here because they are the cleanest cuts and free up the most surface area.

### 1a. Shared Sessions / Pair Programming

**Delete:**
- `app/src/terminal/shared_session/` — entire directory (21 files)
  - Includes: `manager.rs`, `network/`, `permissions_manager.rs`, `presence_manager.rs`, `share_modal/`, `role_change_modal/`, `participant_avatar_view.rs`, `sharer/`, `viewer/`, `replay_agent_conversations.rs`, `selections.rs`, `settings.rs`

**Remove references from:**
- Any `mod shared_session` declarations in `app/src/terminal/mod.rs`
- Any toolbar or context-menu entry points that open the share modal
- Feature flags: `FeatureFlag::CreatingSharedSessions`, `FeatureFlag::ViewingSharedSessions`, `FeatureFlag::SessionSharingAcls`
- Server experiment entries: `ServerExperiment::SessionSharingExperiment`, `ServerExperiment::SessionSharingControl`

### 1b. Team Management

**Delete:**
- `app/src/workspaces/team.rs`
- `app/src/workspaces/gql_convert.rs` (GraphQL team conversions)
- `app/src/server/server_api/team.rs`

**Stub or simplify:**
- `app/src/workspaces/user_workspaces.rs` — Remove team workspace logic. Retain a single personal workspace backed by local SQLite. The `personal_drive()` method should always return a valid local workspace without a user ID check.
- `app/src/workspaces/workspace.rs` — Simplify to remove team-tier concepts (`is_free_team`, paid-gating).
- `app/src/workspaces/user_profiles.rs` — Remove cloud profile fetching; return a static local profile using the OS username.

**Remove references to:**
- Feature flags: `FeatureFlag::TeamApiKeys`, `FeatureFlag::FreeUserNoAi`, `FeatureFlag::FreeUserNoAiControl`
- Server experiments: `FreeUserNoAiControl`, `FreeUserNoAiExperiment`
- Team workspace switching UI in settings and any menus.

### 1c. Billing and Pricing

**Delete:**
- `app/src/billing/` — all upsell and denial modals (e.g. `shared_objects_creation_denied_modal.rs`)
- `app/src/pricing/mod.rs` — entire pricing/plan model

**Remove references to:**
- `FeatureFlag::UsageBasedPricing` and all plan/overage/addon credit feature flags
- `WARP_ERROR_CODE_OUT_OF_CREDITS`, `WARP_ERROR_CODE_AT_CAPACITY` error handling in `server_api.rs`
- The request multiplier and credit tracking logic in `server/server_api.rs`
- `app/src/ai/request_usage_model.rs` (cloud request quota tracking)

---

## Phase 2 — Remove Cloud Sync, Keep Local Data

These features have useful local equivalents. The strategy is: keep the UI and local SQLite storage, remove the cloud sync layer.

### 2a. Cloud Preferences Sync (Settings Sync)

Settings already have a local `settings.toml` fallback. The cloud sync layer is an optional wrapper around it.

**Delete:**
- `app/src/settings/cloud_preferences_syncer.rs` — the sync loop and conflict resolution logic
- `app/src/settings/cloud_preferences.rs` — cloud object wrapper for preferences

**Retain:**
- Local `settings.toml` read/write.
- Platform-specific preference storage without cloud IDs or revision tracking.

**Wire up:** On settings write, write to `settings.toml` only. Remove all calls to `CloudPreferencesSyncer` on startup and on settings change.

### 2b. Warp Drive — Local-Only Mode

Workflows, Notebooks, Env Var Collections, and MCP configs are useful locally. Remove cloud sync; keep local SQLite.

**Delete from `app/src/drive/`:**
- `sharing/` — sharing dialogs and ACLs
- `cloud_action_confirmation_dialog.rs` — cloud conflict UX
- `grab_edit_access_modal.rs` in `app/src/cloud_object/` — "grab the baton" multi-user editing

**Delete from `app/src/server/server_api/`:**
- `object.rs` — cloud object CRUD API
- `workspace.rs` — workspace API

**Simplify `app/src/cloud_object/model/`:**
- Remove revision tracking, server IDs, and `CloudObject` sync trait.
- Replace with a simple local CRUD model backed by SQLite rows (the schema is already there).
- Keep the `ObjectType` enum and JSON serialization — these are useful for local persistence.

**Remove feature flags:**
- `FeatureFlag::SharedWithMe`, `FeatureFlag::WorkflowAliases`, `FeatureFlag::DriveObjectsAsContext` (context menu sharing)

**Result:** Drive panel shows local objects only. No sharing. No "Shared with Me" section.

### 2c. AI Conversation History — Local-Only

Agent conversations are currently synced to cloud. Keep them locally in SQLite.

**Delete:**
- `app/src/ai/persisted_workspace.rs` — cloud-synced conversation metadata

**Simplify:**
- `app/src/ai/agent_conversations_model.rs` — load/save from local SQLite, remove cloud fetch in `can_fetch_agent_runs_for_management`.

**Remove feature flags:**
- `FeatureFlag::AIMemories` (cloud-backed memory store — remove or stub as a local-only facts list)

---

## Phase 3 — Authentication: Replace with Local Identity

The auth system is deeply embedded. The fastest path to local-first is expanding the existing `skip_login` Cargo feature, which already creates a `Credentials::Test` + `User::test()` identity that bypasses Firebase entirely.

### 3a. Add a `local` Cargo Feature

- Add a `local` Cargo feature alongside the existing `skip_login` feature.
- In `app/src/auth/auth_state.rs`: extend `should_use_test_user()` to return `true` when `cfg!(feature = "local")`.
- In `app/src/bin/oss.rs`: enable the `local` feature for the OSS channel build.

This means the OSS build starts fully authenticated as a local user with no Firebase round-trip on launch.

### 3b. Stub `User::local()` to Replace `User::test()`

- `User::test()` exists for tests and carries test-specific semantics. Create `User::local()` that returns a user whose identity comes from the OS (username from `$USER` / `whoami`, no email, no Firebase UID — use a stable UUID derived from machine ID).
- `UserUid` for the local user should be a deterministic UUID from `machine-uid` or a UUID stored in `~/.warp/local_user_id`.

### 3c. Delete Firebase Integration

Once the local user path is in place and no code paths require a real Firebase token:

**Delete:**
- `crates/firebase/` — entire crate
- `app/src/auth/credentials.rs` `Credentials::Firebase` variant and refresh-token logic
- `app/src/auth/web_handoff.rs` — OAuth browser redirect
- `app/src/auth/paste_auth_token_modal.rs`
- `app/src/auth/login_slide.rs` — onboarding login slide
- `app/src/auth/auth_view_modal.rs`, `auth_view_body.rs`, `auth_view_shared_helpers.rs`
- `app/src/auth/needs_sso_link_view.rs`
- `app/src/auth/login_error_modal.rs`, `login_failure_notification.rs`
- `app/src/auth/auth_override_warning_*.rs`

**Retain:**
- `app/src/auth/auth_state.rs` — simplified to local identity only (no Firebase tokens, no anonymous user type, no SSO link state)
- `app/src/auth/auth_manager.rs` — keep a thin wrapper that just returns the local `AuthState`
- `app/src/auth/anonymous_id.rs` → rename to `local_user_id.rs`, repurpose for stable machine UUID

**Remove feature flags:**
- `FeatureFlag::APIKeyAuthentication`, `FeatureFlag::APIKeyManagement` (still support BYO API keys, but as a plain config value, not a cloud-managed feature)
- `FeatureFlag::SkipFirebaseAnonymousUser`

### 3d. Keep API Key Auth for BYO AI

Users who want Warp AI with their own Anthropic/OpenAI/Google keys need to enter those keys somewhere. Keep the API key settings panel, but store keys in `settings.toml` under `[ai_keys]` rather than syncing them to cloud.

---

## Phase 4 — AI: Local Provider Configuration

The AI module has cloud dependencies in two places: (1) the server controls which models are available and at what pricing tier, and (2) conversation history is cloud-synced. Phase 2 handles (2). This phase handles (1).

### 4a. Hardcode Available Models

**Delete:**
- Server-fetched model availability in `app/src/ai/llms.rs` — the `RequiresUpgrade` and `AtCapacity` disable reasons
- `app/src/ai/request_usage_model.rs` — cloud quota tracking (already removed in Phase 1c)

**Replace with:**
- A static list of models that are "available" when the corresponding BYO API key is configured in settings.
- If no key for a provider is set, show the model as "configure API key" rather than "requires upgrade".

### 4b. Remove Cloud Environments

**Delete:**
- `app/src/ai/cloud_environments/` — remote execution feature

**Remove feature flags:**
- `FeatureFlag::CloudEnvironments`, `FeatureFlag::CloudMode`, `FeatureFlag::CloudModeInputV2`, `FeatureFlag::CloudModeHostSelector`, `FeatureFlag::CloudModeSetupV2`
- `FeatureFlag::OzHandoff` (workspace snapshot upload for cloud handoff)
- `FeatureFlag::FullSourceCodeEmbedding` (cloud codebase indexing — remove or stub as local-only grep)

### 4c. Remove AI Analytics

**Delete:**
- `crates/ai/src/telemetry.rs` — conversation telemetry
- `FeatureFlag::AgentModeAnalytics`, `FeatureFlag::GlobalAIAnalyticsCollection`

---

## Phase 5 — Server Infrastructure Cleanup

Once the above phases are complete, the server communication layer can be stripped down significantly.

### 5a. Remove Server API Clients

Once all cloud-backed features are gone, the following server API modules can be deleted:
- `app/src/server/server_api/auth.rs` — Firebase token exchange
- `app/src/server/server_api/team.rs` — already removed in Phase 1b
- `app/src/server/server_api/object.rs` — already removed in Phase 2b
- `app/src/server/server_api/workspace.rs` — already removed in Phase 2b
- `app/src/server/server_api/block.rs` — evaluate whether block execution still requires cloud; if the block runner is local, remove

**Simplify `app/src/server/server_api/ai.rs`:**
- Remove Warp-proxied AI calls. Calls go directly to provider APIs using BYO keys.
- Remove the request multiplier, credit deduction, and capacity-check logic.

### 5b. Remove GraphQL Client

**Delete:**
- `app/src/server/graphql/` — GraphQL client setup
- `crates/warp_graphql_schema/` — GraphQL schema definitions
- `crates/graphql/` — GraphQL utilities
- `crates/warp_server_client/` — server protocol types (or gut it to just the AI streaming protocol if that is used for BYO keys)

### 5c. Remove Server Experiments

The server experiments system (`app/src/server/experiments/`) allows the Warp server to remotely toggle feature flags. With no server connection, this becomes a no-op.

**Delete:**
- `app/src/server/experiments/` — entire module
- All `ServerExperiment` enum variants in `crates/warp_features/`

### 5d. Remove Sync Queue

**Delete:**
- `crates/warp_core/src/sync_queue.rs` and `sync_queue_tests.rs` — batched cloud operation queue

### 5e. Telemetry

**Delete:**
- `app/src/server/telemetry/` — server-side event collection
- `crates/warp_core/src/telemetry.rs` — telemetry infrastructure that ships events to Warp

**Retain (optional):**
- The `FeatureFlag::SendTelemetryToFile` path, repurposed as a local debug log.

---

## Phase 6 — Feature Flag Simplification

`crates/warp_features/src/lib.rs` has ~847 variants. Most of the server-experiment-controlled and cloud-feature flags become dead code once the above phases are complete.

### Strategy:
1. After each phase, do a pass to remove the feature flag variants that gate deleted code.
2. Any remaining feature flags that are "always true" (enabled for OSS builds) should be inlined and the flag deleted.
3. Any remaining flags that are "always false" should have their guarded code deleted, then the flag removed.
4. Retain feature flags that gate genuinely in-progress or toggleable local features.

### Flags to eliminate after Phase 1:
`CreatingSharedSessions`, `ViewingSharedSessions`, `SessionSharingAcls`, `TeamApiKeys`, `FreeUserNoAi`, `UsageBasedPricing`, and all billing/plan/overage variants.

### Flags to eliminate after Phase 3:
`APIKeyAuthentication`, `APIKeyManagement`, `SkipFirebaseAnonymousUser`, and all anonymous-user-gating variants.

### Flags to eliminate after Phase 4:
`CloudEnvironments`, `CloudMode`, `CloudModeInputV2`, `CloudModeHostSelector`, `CloudModeSetupV2`, `OzHandoff`, `FullSourceCodeEmbedding`, `AgentModeAnalytics`, `GlobalAIAnalyticsCollection`, `AIMemories`.

---

## Phase 7 — OSS Build Hardening

Once Phases 1–6 are complete, the OSS build target (`app/src/bin/oss.rs`) should be updated to:

1. Enable the `local` Cargo feature so it starts without login.
2. Remove `WarpServerConfig::production()` — replace with a no-op server config or remove the field.
3. Set `telemetry_config: None` (already done), `oz_config: None` (remove OZ handoff config).
4. Add a README or startup message explaining BYO API key setup for AI features.

---

## Recommended Sequence

| Phase | Effort | Risk | Unlocks |
|-------|--------|------|---------|
| 1a — Shared Sessions | Medium | Low | Clean deletion, no stubs needed |
| 1b — Team Management | Medium | Medium | Requires stubbing `UserWorkspaces` first |
| 1c — Billing/Pricing | Small | Low | Removes upsell noise throughout |
| 2a — Settings Sync | Small | Low | Settings already have local fallback |
| 3a/3b — Local Identity | Small | Medium | Unlocks login-free startup |
| 2b — Drive Local-Only | Medium | Low | Requires local identity first |
| 2c — AI Conversations Local | Small | Low | Requires local identity first |
| 4a — Hardcode Models | Small | Low | Remove cloud model gating |
| 4b — Cloud Environments | Small | Low | Clean deletion |
| 3c — Delete Firebase | Large | High | Do last; many transitive deps |
| 5a-5e — Server Infrastructure | Large | Medium | Do after Firebase removal |
| 6 — Feature Flag Cleanup | Medium | Low | Do incrementally after each phase |
| 7 — OSS Build Hardening | Small | Low | Final integration pass |

Start with Phase 1a (shared sessions) — it is the cleanest cut with zero risk of breakage elsewhere and establishes the pattern for the rest of the work.
