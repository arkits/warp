use futures::channel::oneshot::{self, Receiver};
use warpui::{Entity, ModelContext, SingletonEntity};

/// Local-first compatibility model for old workspace/team refresh call sites.
pub struct TeamUpdateManager;

impl TeamUpdateManager {
    pub fn new(
        _model_event_sender: Option<std::sync::mpsc::SyncSender<crate::persistence::ModelEvent>>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
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

    pub fn stop_polling_for_workspace_metadata_updates(&mut self) {}
}

impl Entity for TeamUpdateManager {
    type Event = ();
}

impl SingletonEntity for TeamUpdateManager {}
