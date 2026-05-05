use super::user_workspaces::UserWorkspaces;
use super::workspace::WorkspaceUid;
use crate::persistence::ModelEvent;
use anyhow::Context;
use futures::channel::oneshot::{self, Receiver};
use std::sync::mpsc::SyncSender;
use warpui::{Entity, ModelContext, SingletonEntity};

pub enum TeamUpdateManagerEvent {
    LeaveSuccess,
    LeaveError,
    RenameTeamSuccess,
    RenameTeamError,
}

/// Local-first compatibility model for old workspace/team refresh call sites.
pub struct TeamUpdateManager {
    model_event_sender: Option<SyncSender<ModelEvent>>,
}

impl TeamUpdateManager {
    pub fn new(
        model_event_sender: Option<SyncSender<ModelEvent>>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self { model_event_sender }
    }

    #[cfg(test)]
    pub fn mock(ctx: &mut ModelContext<Self>) -> Self {
        Self::new(Default::default(), ctx)
    }

    pub fn refresh_workspace_metadata(&mut self, _ctx: &mut ModelContext<Self>) -> Receiver<()> {
        let (tx, rx) = oneshot::channel::<()>();
        let _ = tx.send(());
        rx
    }

    pub fn start_polling_for_workspace_metadata_updates(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn stop_polling_for_workspace_metadata_updates(&mut self) {}

    pub fn set_current_workspace_uid(
        &mut self,
        workspace_uid: WorkspaceUid,
        ctx: &mut ModelContext<Self>,
    ) {
        UserWorkspaces::handle(ctx).update(ctx, |user_workspaces, ctx| {
            user_workspaces.set_current_workspace_uid(workspace_uid, ctx);
        });

        if let Some(model_event_sender) = &self.model_event_sender {
            let _ = model_event_sender
                .send(ModelEvent::SetCurrentWorkspace { workspace_uid })
                .context("Unable to save current workspace to sqlite");
        }
    }
}

impl Entity for TeamUpdateManager {
    type Event = TeamUpdateManagerEvent;
}

impl SingletonEntity for TeamUpdateManager {}
