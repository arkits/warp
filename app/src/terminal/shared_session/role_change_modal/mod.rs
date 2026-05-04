use session_sharing_protocol::common::{ParticipantId, Role, RoleRequestId, RoleRequestResponse};

#[derive(Debug, Clone)]
pub enum RoleChangeOpenSource {
    ViewerRequest { role: Role },
    SharerResponse {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    SharerGrant { participant_id: ParticipantId },
}

#[derive(Debug, Clone, Copy)]
pub enum RoleChangeCloseSource {
    ViewerRequest,
    SharerResponse,
    SharerGrant,
}

#[derive(Clone, Debug)]
pub enum RoleChangeModalEvent {
    Respond {
        request_id: RoleRequestId,
        response: RoleRequestResponse,
    },
    MakeAllReaders,
}
