#[derive(Default)]
pub struct SessionPermissionsManager;

impl SessionPermissionsManager {
    pub fn new<T>(_: &mut warpui::ModelContext<T>) -> Self {
        Self
    }
}

impl warpui::Entity for SessionPermissionsManager {
    type Event = ();
}

impl warpui::SingletonEntity for SessionPermissionsManager {}
