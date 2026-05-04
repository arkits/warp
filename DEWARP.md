# DEWARP Branch

This branch removes internal-only dependencies to make the Warp repository more accessible to external contributors.

## Changes

### Removed Dependencies

- **gcloud CLI**: Removed from bootstrap and install scripts across macOS, Linux, and Windows
- **Channel Config**: Removed `install_channel_config` calls from build scripts

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

## Behavior for External Contributors

- Scripts fail gracefully with clear error messages
- Font fallbacks must be managed manually
- Builds always use `warp-oss` binary