use uuid::Uuid;
use warp_core::user_preferences::GetUserPreferences;

use super::UserUid;

/// Key used to persist the anonymous id to user defaults. We use "ExperimentId" as the key
/// since we use the persisted id to determine experiment groups, and we want to avoid
/// associating it with telemetry.
const ANONYMOUS_ID_KEY: &str = "ExperimentId";
const LOCAL_USER_ID_KEY: &str = "LocalUserId";

/// Reads the persisted anonymous id from user defaults, if it exists and is a
/// valid uuid.
fn get_persisted_anonymous_id(ctx: &dyn GetUserPreferences) -> Option<Uuid> {
    let anonymous_id = ctx
        .private_user_preferences()
        .read_value(ANONYMOUS_ID_KEY)
        .unwrap_or_default()?;
    match Uuid::parse_str(&anonymous_id) {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            log::warn!("Error parsing persisted anonymous id from user defaults: {e:?}");
            None
        }
    }
}

/// Gets the persisted anonymous id if possible, otherwise generates a new uuid
/// and saves it to user defaults.
pub fn get_or_create_anonymous_id(ctx: &dyn GetUserPreferences) -> Uuid {
    get_persisted_anonymous_id(ctx).unwrap_or_else(|| {
        let uuid = Uuid::new_v4();
        let _ = ctx
            .private_user_preferences()
            .write_value(ANONYMOUS_ID_KEY, uuid.to_string());
        uuid
    })
}

/// Returns a stable local user ID derived from the OS username. On first call the UUID is
/// generated and persisted to user preferences so it survives restarts.
pub fn get_or_create_local_user_id(ctx: &dyn GetUserPreferences) -> UserUid {
    let stored = ctx
        .private_user_preferences()
        .read_value(LOCAL_USER_ID_KEY)
        .unwrap_or_default();
    if let Some(uid) = stored {
        return UserUid::new(&uid);
    }
    let uuid = Uuid::new_v4();
    let uid = format!(
        "local:{}:{}",
        std::env::var("USER").unwrap_or_else(|_| "user".to_string()),
        uuid
    );
    let _ = ctx
        .private_user_preferences()
        .write_value(LOCAL_USER_ID_KEY, uid.clone());
    UserUid::new(&uid)
}
