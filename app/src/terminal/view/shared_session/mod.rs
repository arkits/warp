use chrono::{DateTime, Local};
use session_sharing_protocol::common::{SessionId, WindowSize};
use warpui::elements::MouseStateHandle;
use warpui::prelude::Container;
use warpui::{AnyView, AppContext, Element, ModelHandle, ViewHandle};

use crate::terminal::shared_session::presence_manager::PresenceManager;

pub mod adapter {
    use super::{Sharer, Viewer};

    pub enum Kind {
        Viewer(Viewer),
        Sharer(Sharer),
    }

    impl Kind {
        pub fn as_viewer(&self) -> Option<&Viewer> {
            match self {
                Self::Viewer(viewer) => Some(viewer),
                Self::Sharer(_) => None,
            }
        }
    }
}

pub use adapter::Kind;

pub struct SharedSessionAdapter {
    session_id: SessionId,
    started_at: DateTime<Local>,
    presence_manager: ModelHandle<PresenceManager>,
    kind: Kind,
}

impl SharedSessionAdapter {
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn started_at(&self) -> &DateTime<Local> {
        &self.started_at
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn presence_manager(&self) -> &ModelHandle<PresenceManager> {
        &self.presence_manager
    }

    pub fn pane_header_viewer_avatars(&self, _: &AppContext) -> Vec<Box<dyn AnyView>> {
        Vec::new()
    }

    pub fn presence_avatars(&self, _: &AppContext) -> Vec<Box<dyn AnyView>> {
        Vec::new()
    }

    pub fn reconnecting_banner(&self) -> Option<Box<dyn Element>> {
        None
    }
}

pub struct ParticipantAvatar {
    pub avatar: ViewHandle<Container>,
}

pub struct Viewer {
    pub sharer: Option<ParticipantAvatar>,
    pub sharer_size: Option<WindowSize>,
    pub last_reported_natural_size: Option<(usize, usize)>,
    pub role_change_menu: ViewHandle<Container>,
    pub is_role_change_menu_open: bool,
    pub role_change_menu_button: MouseStateHandle,
}

pub struct Sharer {
    revoke_all_mouse_state_handle: MouseStateHandle,
}

impl Sharer {
    pub fn revoke_all_mouse_state_handle(&self) -> &MouseStateHandle {
        &self.revoke_all_mouse_state_handle
    }

    pub fn is_inactivity_warning_modal_open(&self) -> bool {
        false
    }
}
