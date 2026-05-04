use session_sharing_protocol::common::SessionId;
use warpui::{AppContext, ModelHandle};

use crate::terminal::TerminalView;

#[derive(Default)]
pub struct Manager;

impl warpui::Entity for Manager {
    type Event = ManagerEvent;
}

impl warpui::SingletonEntity for Manager {}

#[allow(dead_code)]
pub enum ManagerEvent {
    CopyLink(SessionId),
}

impl Manager {
    pub fn new<T>(_: &mut warpui::ModelContext<T>) -> Self {
        Self
    }

    pub fn as_ref(_: &AppContext) -> &'static Self {
        static MANAGER: Manager = Manager;
        &MANAGER
    }

    pub fn shared_views(&self, _: &AppContext) -> std::iter::Empty<ModelHandle<TerminalView>> {
        std::iter::empty()
    }
}
