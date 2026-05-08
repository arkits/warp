use std::path::PathBuf;

use uuid::Uuid;

use warp_util::path::EscapeChar;
use warpui::{App, EntityId, ModelHandle};

use warp_core::execution_mode::ExecutionMode;

use crate::ai::active_agent_views_model::ActiveAgentViewsModel;
use crate::terminal::cli_agent_sessions::CLIAgentSessionsModel;
use crate::{
    ai::{
        agent::conversation::AIConversationId,
        blocklist::{
            permissions::{
                CommandExecutionPermission, CommandExecutionPermissionDeniedReason,
                FileReadPermission, FileReadPermissionAllowedReason,
                FileReadPermissionDeniedReason, FileWritePermission,
                FileWritePermissionAllowedReason, FileWritePermissionDeniedReason,
            },
            CommandExecutionPermissionAllowedReason,
        },
        execution_profiles::{
            profiles::AIExecutionProfilesModel, ActionPermission, WriteToPtyPermission,
        },
        mcp::templatable_manager::TemplatableMCPServerManager,
    },
    auth::AuthStateProvider,
    cloud_object::model::persistence::CloudModel,
    network::NetworkStatus,
    server::{cloud_objects::update_manager::UpdateManager, sync_queue::SyncQueue},
    settings::{AgentModeCommandExecutionPredicate, PrivacySettings},
    test_util::settings::initialize_settings_for_tests_with_mode,
    workspaces::{
        team_tester::TeamTesterStatus, user_workspaces::UserWorkspaces,
        workspace::SandboxedAgentSettings,
    },
    AgentNotificationsModel, GlobalResourceHandles, GlobalResourceHandlesProvider, LaunchMode,
};

use super::{BlocklistAIHistoryModel, BlocklistAIPermissions};

struct PermissionsTestState {
    convo_id: AIConversationId,
    permissions: ModelHandle<BlocklistAIPermissions>,
    history: ModelHandle<BlocklistAIHistoryModel>,
    terminal_view_id: EntityId,
    user_workspaces: ModelHandle<UserWorkspaces>,
    profile_model: ModelHandle<AIExecutionProfilesModel>,
}

fn initialize_permissions_test(app: &mut App) -> PermissionsTestState {
    initialize_permissions_test_with_mode(app, ExecutionMode::App, false)
}

fn initialize_permissions_test_sandboxed(app: &mut App) -> PermissionsTestState {
    let state = initialize_permissions_test_with_mode(app, ExecutionMode::Sdk, true);
    state.profile_model.update(app, |model, ctx| {
        let profile_id = *model.default_profile(ctx).id();
        model.apply_cli_profile_defaults_for_test(profile_id, true, ctx);
    });
    state
}

fn initialize_permissions_test_with_mode(
    app: &mut App,
    mode: ExecutionMode,
    is_sandboxed: bool,
) -> PermissionsTestState {
    initialize_settings_for_tests_with_mode(app, mode, is_sandboxed);
    let global_resource_handles = GlobalResourceHandles::mock(app);
    app.add_singleton_model(|_| GlobalResourceHandlesProvider::new(global_resource_handles));
    let history = app.add_singleton_model(|_| BlocklistAIHistoryModel::new(vec![], &[]));
    app.add_singleton_model(|_| CLIAgentSessionsModel::new());
    app.add_singleton_model(|_| ActiveAgentViewsModel::new());
    app.add_singleton_model(AgentNotificationsModel::new);
    let permissions = app.add_singleton_model(BlocklistAIPermissions::new);
    let terminal_view_id = EntityId::new();
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(SyncQueue::mock);
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(TeamTesterStatus::mock);
    app.add_singleton_model(UpdateManager::mock);
    app.add_singleton_model(CloudModel::mock);
    app.add_singleton_model(|_| TemplatableMCPServerManager::default());
    let profile_model = app.add_singleton_model(|ctx| {
        AIExecutionProfilesModel::new(&LaunchMode::new_for_unit_test(), ctx)
    });
    app.add_singleton_model(PrivacySettings::mock);
    let user_workspaces = app.add_singleton_model(UserWorkspaces::default_mock);

    let conversation_id = history.update(app, |history_model, ctx| {
        history_model.start_new_conversation(terminal_view_id, false, false, ctx)
    });

    PermissionsTestState {
        convo_id: conversation_id,
        permissions,
        history,
        terminal_view_id,
        user_workspaces,
        profile_model,
    }
}

#[test]
fn test_can_read_files_empty_paths() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(FileReadPermissionAllowedReason::ExplicitlyAllowlisted)
            ));
        });
    })
}

#[test]
fn test_can_write_files() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            terminal_view_id,
            convo_id,
            permissions,
            profile_model,
            ..
        } = initialize_permissions_test(&mut app);

        // Test AgentDecides setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(!result.is_allowed());
            assert!(
                matches!(
                    result,
                    FileWritePermission::Denied(FileWritePermissionDeniedReason::AgentDecided)
                ),
                "not allowed because AgentDecides right now just means ask"
            );
        });

        // Test AlwaysAllow setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Allowed(
                    FileWritePermissionAllowedReason::AutowriteSettingEnabled
                )
            ));
        });

        // Test AlwaysAsk setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Denied(FileWritePermissionDeniedReason::AlwaysAskEnabled)
            ));
        });
    })
}

#[test]
fn test_can_write_files_mcp_config_always_denied() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            terminal_view_id,
            convo_id,
            permissions,
            profile_model,
            ..
        } = initialize_permissions_test(&mut app);

        // Even with AlwaysAllow, writing to an MCP config must be denied.
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        let mcp_config_paths = vec![
            PathBuf::from("/project/.mcp.json"),
            PathBuf::from("/project/.warp/.mcp.json"),
            PathBuf::from("/project/.codex/config.toml"),
        ];

        for path in mcp_config_paths {
            permissions.read(&app, |model, ctx| {
                let result = model.can_write_files(
                    &convo_id,
                    std::slice::from_ref(&path),
                    Some(terminal_view_id),
                    ctx,
                );
                assert!(
                    !result.is_allowed(),
                    "expected MCP config path {path:?} to be denied"
                );
                assert!(
                    matches!(
                        result,
                        FileWritePermission::Denied(FileWritePermissionDeniedReason::ProtectedPath)
                    ),
                    "expected ProtectedPath denial for {path:?}, got {result:?}"
                );
            });
        }
    })
}

#[test]
fn test_can_autoexecute_command_denylist_beats_run_to_completion() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            history,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Add a denylist rule that matches the test command.
        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        // Toggle run-to-completion override for this conversation.
        history.update(&mut app, |history, ctx| {
            history.toggle_autoexecute_override(&convo_id, terminal_view_id, ctx);
        });

        // Despite run-to-completion, denylist must take precedence and deny execution.
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm important.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });
    })
}

#[test]
fn test_can_autoexecute_command_run_to_completion_allows_non_denylisted() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            history,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Enable run-to-completion override for the conversation.
        history.update(&mut app, |history, ctx| {
            history.toggle_autoexecute_override(&convo_id, terminal_view_id, ctx);
        });

        // Since the command is not denylisted, the override should allow execution with RunToCompletion.
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "echo hello",
                EscapeChar::Backslash,
                true,        // read-only command
                Some(false), // not risky
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::RunToCompletion
                )
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_allow_no_denylist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Any server should be allowed when AlwaysAllow and not denylisted.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // None UUID should also be allowed (no denylist match possible).
            assert!(model.can_use_mcp_server(&convo_id, None, Some(terminal_view_id), ctx));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_allow_with_denylist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Denylisted server should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Non-denylisted server should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_ask_with_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Allowlisted server should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Non-allowlisted server should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // None UUID should be denied.
            assert!(!model.can_use_mcp_server(&convo_id, None, Some(terminal_view_id), ctx));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_ask_denylist_overrides_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Both allowlisted and denylisted: denylist wins.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_agent_decides() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Allowlisted and not denylisted should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Not allowlisted should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_agent_decides_denylist_overrides_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Both allowlisted and denylisted: denylist wins.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_sandboxed_mode_allows_read_write_files() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            terminal_view_id,
            ..
        } = initialize_permissions_test_sandboxed(&mut app);

        // Set workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.apply_code_diffs_setting = Some(ActionPermission::AlwaysAsk);
                    settings.read_files_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        // In sandboxed mode the workspace read/write restrictions are bypassed,
        // so the profile's AlwaysAllow setting takes effect.
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(
                result.is_allowed(),
                "write files should be allowed in sandboxed mode (workspace restriction bypassed)"
            );
            assert!(matches!(
                result,
                FileWritePermission::Allowed(
                    FileWritePermissionAllowedReason::AutowriteSettingEnabled
                )
            ));

            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/test/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                result.is_allowed(),
                "read files should be allowed in sandboxed mode (workspace restriction bypassed)"
            );
            assert!(matches!(
                result,
                FileReadPermission::Allowed(
                    FileReadPermissionAllowedReason::AutoreadSettingEnabled
                )
            ));
        });
    })
}

#[test]
fn test_empty_org_denylist_allows_user_entries() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist = Some(vec![]);
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "user denylist entry should be active even when org denylist is empty"
            );
        });
    })
}

#[test]
fn test_denylist_matches_multiline_commands() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Add denylist rule for rm
        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        // Single-line rm command should be denied
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Multiline rm command with backslash continuations should also be denied (POSIX)
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file1.txt \\\nfile2.txt \\\nfile3.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "multiline rm command should be denied by denylist"
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Multiline rm command with backtick continuations should also be denied (PowerShell)
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file1.txt `\nfile2.txt `\nfile3.txt",
                EscapeChar::Backtick,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "multiline rm command with backtick continuations should be denied by denylist"
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });
    })
}
