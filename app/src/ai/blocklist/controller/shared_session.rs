use session_sharing_protocol::common::ParticipantId;

#[derive(Default)]
pub struct SharedSessionState;

impl SharedSessionState {
    pub fn get_current_response_initiator(&self) -> Option<ParticipantId> {
        None
    }

    pub fn set_current_response_initiator(&mut self, _participant_id: ParticipantId) {}

    pub fn get_sharer_participant_id(&self) -> Option<ParticipantId> {
        None
    }
}
