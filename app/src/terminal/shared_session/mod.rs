//! Local-first compatibility surface for code paths that are being unwired from
//! session sharing. All state resolves to a non-shared local terminal.

use serde::{Deserialize, Serialize};
use session_sharing_protocol::common::{Role, SessionId};
use session_sharing_protocol::sharer::SessionSourceType;
use warpui::{id, keymap::ContextPredicate};

use super::model::terminal_model::BlockIndex;

pub mod manager;
pub mod participant_avatar_view;
pub mod permissions_manager;
pub mod presence_manager;
pub mod render_util;
pub mod role_change_modal;
pub mod viewer;

#[derive(Debug, Clone, Default)]
pub enum IsSharedSessionCreator {
    Yes { source_type: SessionSourceType },
    #[default]
    No,
}

#[derive(Debug, Clone)]
pub enum SharedSessionStatus {
    NotShared,
    ViewPending,
    ActiveViewer { role: Role },
    FinishedViewer,
    SharePendingPreBootstrap { source_type: SessionSourceType },
    SharePending,
    ActiveSharer,
}

impl PartialEq for SharedSessionStatus {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for SharedSessionStatus {}

impl From<&Role> for crate::editor::InteractionState {
    fn from(value: &Role) -> crate::editor::InteractionState {
        match value {
            Role::Reader => crate::editor::InteractionState::Selectable,
            Role::Executor | Role::Full => crate::editor::InteractionState::Editable,
        }
    }
}

impl SharedSessionStatus {
    pub fn reader() -> Self {
        Self::ActiveViewer { role: Role::Reader }
    }

    pub fn executor() -> Self {
        Self::ActiveViewer {
            role: Role::Executor,
        }
    }

    pub fn is_view_pending(&self) -> bool {
        matches!(self, Self::ViewPending)
    }

    pub fn is_active_viewer(&self) -> bool {
        matches!(self, Self::ActiveViewer { .. })
    }

    pub fn is_finished_viewer(&self) -> bool {
        matches!(self, Self::FinishedViewer)
    }

    pub fn is_viewer(&self) -> bool {
        self.is_view_pending() || self.is_active_viewer() || self.is_finished_viewer()
    }

    pub fn is_executor(&self) -> bool {
        matches!(self, Self::ActiveViewer { role } if role.can_execute())
    }

    pub fn is_reader(&self) -> bool {
        matches!(self, Self::ActiveViewer { role: Role::Reader })
    }

    pub fn is_share_pending(&self) -> bool {
        matches!(self, Self::SharePending | Self::SharePendingPreBootstrap { .. })
    }

    pub fn is_active_sharer(&self) -> bool {
        matches!(self, Self::ActiveSharer)
    }

    pub fn is_sharer(&self) -> bool {
        self.is_share_pending() || self.is_active_sharer()
    }

    pub fn is_sharer_or_viewer(&self) -> bool {
        !matches!(self, Self::NotShared)
    }

    pub fn as_keymap_context(&self) -> &'static str {
        match self {
            Self::NotShared => "SharedSessionStatus_NotShared",
            Self::ViewPending => "SharedSessionStatus_ViewPending",
            Self::ActiveViewer { role: Role::Reader } => "SharedSessionStatus_Reader",
            Self::ActiveViewer {
                role: Role::Executor | Role::Full,
            } => "SharedSessionStatus_Executor",
            Self::FinishedViewer => "SharedSessionStatus_FinishedViewer",
            Self::SharePendingPreBootstrap { .. } => "SharedSessionStatus_SharePendingPreBootstrap",
            Self::SharePending => "SharedSessionStatus_SharePending",
            Self::ActiveSharer => "SharedSessionStatus_ActiveSharer",
        }
    }

    pub fn active_viewer_keymap_context() -> ContextPredicate {
        id!(Self::reader().as_keymap_context()) | id!(Self::executor().as_keymap_context())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedSessionScrollbackType {
    None,
    FromBlock { block_index: BlockIndex },
    All,
}

impl SharedSessionScrollbackType {
    pub fn first_block_index(self, _model: &super::TerminalModel) -> BlockIndex {
        match self {
            Self::FromBlock { block_index } => block_index,
            Self::None | Self::All => BlockIndex::zero(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum SharedSessionActionSource {
    BlocklistContextMenu { block_index: Option<BlockIndex> },
    Tab,
    PaneHeader,
    CommandPalette,
    OnboardingBlock,
    Closed { is_confirm_close_session: bool },
    InactivityModal,
    NonUser,
    SharingDialog,
    RightClickMenu,
    FooterChip,
}

pub fn join_link(session_id: &SessionId) -> String {
    format!("local-session-disabled:{session_id}")
}
