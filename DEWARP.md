# DEWARP Branch

This branch removes internal-only dependencies to make the Warp repository more accessible to external contributors.

## Changes

### Removed Dependencies

- **gcloud CLI**: Removed from bootstrap and install scripts across macOS, Linux, and Windows
- **Channel Config**: Removed `install_channel_config` calls from build scripts
- **Referrals Feature**: Removed GraphQL queries/mutations, settings page, theme integration, and assets

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

- **Referrals**: Referrals settings page, GraphQL queries/mutations, referral theme rewards
- `crates/graphql/src/api/queries/get_referral_info.rs`
- `crates/graphql/src/api/mutations/send_referral_invite_emails.rs`
- `app/src/settings_view/referrals_page.rs`
- `app/src/server/server_api/referral.rs`
- `app/src/referral_theme_status.rs`
- Referral SVG assets and reward themes (SentReferralReward, ReceivedReferralReward)

## Behavior for External Contributors

- Scripts fail gracefully with clear error messages
- Font fallbacks must be managed manually
- Builds always use `warp-oss` binary
- Referrals feature is unavailable