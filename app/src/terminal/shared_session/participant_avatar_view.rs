use session_sharing_protocol::common::Role;
use warpui::elements::MouseStateHandle;
use warpui::elements::Empty;
use warpui::prelude::Container;
use warpui::{AnyView, AppContext, Element, ViewHandle};

pub fn render_participants_and_role_elements(
    _: Vec<Box<dyn AnyView>>,
    _: Role,
    _: MouseStateHandle,
    _: Option<ViewHandle<Container>>,
    _: bool,
    _: bool,
    _: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}
