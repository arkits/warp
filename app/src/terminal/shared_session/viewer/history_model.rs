#[derive(Default)]
pub struct SharedSessionHistoryModel {
    entries: Vec<crate::terminal::HistoryEntry>,
}

impl SharedSessionHistoryModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> impl Iterator<Item = &crate::terminal::HistoryEntry> {
        self.entries.iter()
    }

    pub fn push(&mut self, entry: crate::terminal::HistoryEntry) {
        self.entries.push(entry);
    }
}

impl warpui::Entity for SharedSessionHistoryModel {
    type Event = ();
}
