use pathfinder_color::ColorU;
use warpui::elements::Empty;
use warpui::{AnyView, AppContext, Element};

pub fn muted_participant_color() -> ColorU {
    ColorU::new(128, 128, 128, 255)
}

#[derive(Default)]
pub struct PresenceManager;

impl warpui::Entity for PresenceManager {
    type Event = ();
}

impl PresenceManager {
    pub fn is_reconnecting(&self) -> bool {
        false
    }

    pub fn all_present_participants(&self) -> std::iter::Empty<Participant> {
        std::iter::empty()
    }

    pub fn get_participant(&self, _: &session_sharing_protocol::common::ParticipantId) -> Option<Participant> {
        None
    }

    pub fn role(&self) -> session_sharing_protocol::common::Role {
        session_sharing_protocol::common::Role::Full
    }
}

pub struct Participant {
    pub color: ColorU,
    pub info: ParticipantInfo,
    pub role: Option<session_sharing_protocol::common::Role>,
}

pub struct ParticipantInfo {
    pub id: session_sharing_protocol::common::ParticipantId,
    pub selection: session_sharing_protocol::common::Selection,
    pub profile_data: ParticipantProfileData,
}

pub struct ParticipantProfileData {
    pub display_name: String,
    pub photo_url: Option<String>,
}

pub fn text_selection_color(color: ColorU) -> ColorU {
    color
}

pub fn render_participants(_: &PresenceManager, _: &AppContext) -> Vec<Box<dyn AnyView>> {
    Vec::new()
}

pub fn render_participant_role(_: &PresenceManager, _: &AppContext) -> Box<dyn Element> {
    Empty::new().finish()
}
