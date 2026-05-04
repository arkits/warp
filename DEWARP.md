# DEWARP Branch

This branch removes internal-only dependencies to make the Warp repository more accessible to external contributors.

## Changes

### Removed Dependencies

- **gcloud CLI**: Removed from bootstrap and install scripts across macOS, Linux, and Windows
- **Channel Config**: Removed `install_channel_config` calls from build scripts
- **Referrals Feature**: Removed GraphQL queries/mutations, settings page, theme integration, and assets
- **Sentry**: Removed `sentry`, `sentry-log`, `minidumper`, and `crash-handler` crate dependencies; removed `sentry-cli` from bootstrap and CI

### Modified Scripts

| Script | Change |
|--------|--------|
| `script/font_fallback/generate-families.py` | Removed gcloud font listing |
| `script/font_fallback/generate-mappings.py` | Removed gcloud font download, shows error for external |
| `script/install_cargo_build_deps` | Removed channel config installation |
| `script/linux/bootstrap` | Removed gcloud authentication |
| `script/linux/install_test_deps` | Removed gcloud installation |
| `script/macos/bootstrap` | Removed gcloud SDK install + auth |
| `script/run` | Hardcoded `warp-oss` binary |
| `script/windows/bootstrap.ps1` | Removed gcloud SDK install + auth |

### Removed Features

- **Sentry crash reporting**: Removed all Sentry SDK integration (Rust, Cocoa, and minidump). All crash reporting functions are now no-ops. Removed `sentry`, `sentry-log`, `minidumper`, and `crash-handler` crate dependencies.
  - `app/src/crash_reporting/mod.rs`: Rewritten as stubs
  - `app/src/crash_reporting/sentry_minidump.rs`: Rewritten as stub
  - `app/src/crash_reporting/mac.rs`: Rewritten as stub (ObjC Sentry calls removed)
  - `crates/warp_core/src/errors.rs`, `errors/anyhow.rs`: Removed `sentry::capture_error` / `capture_anyhow`
  - `crates/warp_logging/src/native.rs`: Removed `SentryLogger` wrapping
  - `app/src/profiling.rs`, `workspace/view.rs`, `autoupdate/windows.rs`, `terminal/model/session.rs`, `ai/blocklist/controller/response_stream.rs`, `persistence/sqlite.rs`: Removed inline sentry capture calls
  - `app/build.rs`: Removed `build_and_link_sentry` and related functions
  - `script/sentry_upload_dif.sh`, `script/sentry_create_release.sh`: Deleted
  - `script/macos/update_sentry_cocoa`: Deleted
  - `.github/workflows/create_release.yml`: Removed all `setup-sentry-cli`, `getsentry/action-release`, and symbol upload steps
  - `.github/actions/get_channel_config/action.yml`: Removed `sentry_project` and `sentry_environment` outputs
  - `script/macos/bootstrap`: Removed `brew install getsentry/tools/sentry-cli`
  - `script/macos/bundle`: Removed `cocoa_sentry` from default feature set
  - `script/macos/run`: Removed Sentry.framework copy step
  - `script/wasm/install_build_deps`: Removed `wasm-split` download from `getsentry/symbolicator`
  - `script/wasm/bundle`: Removed `wasm-split` invocation

- **Referrals**: Referrals settings page, GraphQL queries/mutations, referral theme rewards
  - `crates/graphql/src/api/queries/get_referral_info.rs`
  - `crates/graphql/src/api/mutations/send_referral_invite_emails.rs`
  - `app/src/settings_view/referrals_page.rs`
  - `app/src/server/server_api/referral.rs`
  - `app/src/referral_theme_status.rs`
  - Referral SVG assets and reward themes (SentReferralReward, ReceivedReferralReward)

- **Settings button menu items**: Removed Slack, Sign Up, and Upgrade from the top-left settings dropdown
  - `app/src/workspace/view.rs`: Removed `JoinSlack`, `SignupAnonymousUser`, and `ShowUpgrade` menu items

## Behavior for External Contributors

- Scripts fail gracefully with clear error messages
- Font fallbacks must be managed manually
- Builds always use `warp-oss` binary
- Referrals feature is unavailable
- Settings button does not show Slack community link, Sign Up, or Upgrade prompts